import 'dockview/dist/styles/dockview.css';
import './style.css';
import { buildHome, log } from './home';
import { subscribe } from './bridge';
import { startTitle, stats } from './stats';
import { startHand } from './hand';

buildHome(document.getElementById('dock')!, document.getElementById('bar')!);
startTitle();
subscribe().then((source) => {
  stats.source = source;
  log(`stream source: ${source}`);
});
startHand(document.getElementById('hand') as HTMLCanvasElement);
