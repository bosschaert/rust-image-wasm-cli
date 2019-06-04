import * as wasm from "imagewasm";

var image;

function toDataURL(url, callback) {
  var xhr = new XMLHttpRequest();
  xhr.onload = function() {
    var reader = new FileReader();
    reader.onloadend = function() {
      callback(reader.result);
    }
    reader.readAsDataURL(xhr.response);
  };
  xhr.open('GET', url);
  xhr.responseType = 'blob';
  xhr.send();
}

function blur() {
  image = wasm.blur(image);
  document.getElementById("IP2").src = image;
}

function invert() {
  image = wasm.invert(image);
  document.getElementById("IP2").src = image;
}

function greyscale() {
  image = wasm.greyscale(image);
  document.getElementById("IP2").src = image;
}

function rotate() {
  image = wasm.rotate(image);
  document.getElementById("IP2").src = image;
}

function vflip() {
  image = wasm.vertical_flip(image);
  document.getElementById("IP2").src = image;
}

function hflip() {
  image = wasm.horizontal_flip(image);
  document.getElementById("IP2").src = image;
}

toDataURL('http://localhost:8080/mrmen.png', function(dataUrl) {
  image = dataUrl;
  // alert("Before: " + dataUrl);
  // var converted = wasm.transform(dataUrl);
  // alert("After: " + converted);
  document.getElementById("IP2").src = image;
})
document.getElementById("blur").onclick = blur;
document.getElementById("invert").onclick = invert;
document.getElementById("greyscale").onclick = greyscale;
document.getElementById("rotate").onclick = rotate;
document.getElementById("hflip").onclick = hflip;
document.getElementById("vflip").onclick = vflip;
