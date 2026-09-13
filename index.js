const welcome_array = ['Welcome stranger!', 'Hey! How -- ah, these pretzels suck...', "Oh... hi. This looks bad. I don't suppose you've seen Abe, have you?"];

window.onload = function () {
  const size = welcome_array.length;
  const index = Math.floor(Math.random() * size);
  document.getElementById('main-label').innerText = welcome_array[index];
};


