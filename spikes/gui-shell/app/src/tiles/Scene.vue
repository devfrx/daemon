<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue';
import { AmbientLight, BoxGeometry, Color, DirectionalLight, InstancedMesh, Matrix4, MeshStandardMaterial, PerspectiveCamera, Scene } from 'three';
import { WebGPURenderer } from 'three/webgpu';
import { stats } from '../stats';

// The 3D tile: a field of instanced cubes under two lights, rendered by WebGPURenderer, which falls back
// to WebGL2 by itself (P-6). M3: fps counted per second, mean and min over the last 30 seconds; the API
// read from the backend the renderer actually obtained, with the adapter or the unmasked renderer string.
defineProps<{ title?: string; api?: unknown; containerApi?: unknown; params?: unknown }>();

const host = ref<HTMLDivElement>();
let stopped = false;
let observer: ResizeObserver | undefined;

async function describeApi(renderer: WebGPURenderer): Promise<string> {
  const backend = (renderer as unknown as { backend?: { isWebGPUBackend?: boolean; isWebGLBackend?: boolean } }).backend;
  if (backend?.isWebGPUBackend) {
    try {
      const gpu = (navigator as unknown as { gpu?: { requestAdapter(): Promise<unknown> } }).gpu;
      const adapter = await gpu?.requestAdapter();
      const info = (adapter as { info?: { vendor?: string; architecture?: string } } | undefined)?.info;
      return `WebGPU:${info?.vendor ?? '?'}/${info?.architecture ?? '?'}`;
    } catch {
      return 'WebGPU:?';
    }
  }
  if (backend?.isWebGLBackend) {
    const gl = document.createElement('canvas').getContext('webgl2');
    const dbg = gl?.getExtension('WEBGL_debug_renderer_info');
    if (gl && dbg) {
      const name = String(gl.getParameter(dbg.UNMASKED_RENDERER_WEBGL)).replace(/\s+/g, '_').slice(0, 40);
      return `WebGL2:${name}`;
    }
    return 'WebGL2:?';
  }
  return 'unknown';
}

onMounted(async () => {
  const el = host.value!;
  const renderer = new WebGPURenderer({ antialias: true });
  await renderer.init();
  stats.api = await describeApi(renderer);
  el.append(renderer.domElement);

  const scene = new Scene();
  scene.background = new Color(0x0b0e14);
  const camera = new PerspectiveCamera(60, 1, 0.1, 200);
  camera.position.set(0, 0, 40);
  scene.add(new AmbientLight(0xffffff, 0.4));
  const sun = new DirectionalLight(0xffffff, 1.2);
  sun.position.set(10, 20, 30);
  scene.add(sun);
  const COUNT = 4000;
  const cubes = new InstancedMesh(new BoxGeometry(1, 1, 1), new MeshStandardMaterial({ color: 0x4f8fdb, roughness: 0.4, metalness: 0.2 }), COUNT);
  const m = new Matrix4();
  for (let i = 0; i < COUNT; i += 1) {
    m.makeTranslation((Math.random() - 0.5) * 60, (Math.random() - 0.5) * 40, (Math.random() - 0.5) * 40);
    cubes.setMatrixAt(i, m);
  }
  scene.add(cubes);

  const fit = () => {
    const w = Math.max(1, el.clientWidth);
    const h = Math.max(1, el.clientHeight);
    renderer.setSize(w, h, false);
    camera.aspect = w / h;
    camera.updateProjectionMatrix();
    stats.scene = `${w}x${h}`;
  };
  fit();
  observer = new ResizeObserver(fit);
  observer.observe(el);

  let frames = 0;
  let windowStart = performance.now();
  const seconds: number[] = [];
  const loop = () => {
    if (stopped) return;
    cubes.rotation.y += 0.004;
    cubes.rotation.x += 0.002;
    renderer.render(scene, camera);
    frames += 1;
    const now = performance.now();
    if (now - windowStart >= 1000) {
      seconds.push((frames * 1000) / (now - windowStart));
      if (seconds.length > 30) seconds.shift();
      stats.fps = seconds.reduce((a, b) => a + b, 0) / seconds.length;
      stats.fpsMin = Math.min(...seconds);
      frames = 0;
      windowStart = now;
    }
    requestAnimationFrame(loop);
  };
  requestAnimationFrame(loop);
});

onUnmounted(() => {
  stopped = true;
  observer?.disconnect();
});
</script>

<template>
  <div ref="host" class="scene"></div>
</template>
