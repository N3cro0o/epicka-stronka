import initMain from './wasm/output/main/web_page_main_canvas_bin.js';
import {check_page, change_page_num} from './wasm/output/main/web_page_main_canvas_bin.js';

const welcome_array = ['Welcome stranger!', 'Hey! How -- ah, these pretzels suck...', "Oh... hi. This looks bad. I don't suppose you've seen Abe, have you?"];

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
