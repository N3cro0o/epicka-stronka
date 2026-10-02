import initMain from './wasm/output/main/web_page_main_canvas_bin.js';
import { check_page, change_page_num, change_app_size } from './wasm/output/main/web_page_main_canvas_bin.js';

const welcome_array = ['Welcome stranger!', 'Hey! How -- ah, these pretzels suck...', "Oh... hi. This looks bad. I don't suppose you've seen Abe, have you?"];

function wasm_resize() {
  document.getElementById("about-me-button").checked = true;
  let c = document.querySelector('canvas');
  let rect = c.getBoundingClientRect();
  console.log('canvas html size ', rect.width, rect.height);
  change_app_size(rect.width, rect.height)
}

window.onload = function () {
  const size = welcome_array.length;
  const index = Math.floor(Math.random() * size);
  document.getElementById('main-label').innerText = welcome_array[index];
};

await initMain();

function radio_button_click(id) {
  check_page();
  change_page_num(id);
}

document.querySelectorAll('input[name="section"]').forEach( (radio, index) => {
  radio.addEventListener('change', (e) => {
    if (e.target.checked) {
      radio_button_click(index);
    }
  });
});

let timeout; 
window.addEventListener('resize', () => {
  console.log("XD");
  clearTimeout(timeout);
  timeout = setTimeout(wasm_resize, 100);
});

const waitForCanvas = setInterval(() => {
  if (document.querySelector('canvas')) {
    clearInterval(waitForCanvas);
    wasm_resize(); 
  }
}, 200);
