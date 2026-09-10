import './style.css';
import { buildHome, log } from './home';

buildHome(document.getElementById('dock')!, document.getElementById('bar')!);
log('SP-8 Home: moves 1-7 in this browser; the live tiles arrive with task 3');
