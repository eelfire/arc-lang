// example taken from :
const fs = require('fs');
const path = require('path');
(async () => {
    const file = await fs.promises.readFile(path.join(__dirname, 'wasm', 'caesar.wasm'));
    const wasm = await WebAssembly.instantiate(file);
    const { memory, caesarEncrypt, caesarDecrypt } = wasm.instance.exports;

    const encode = function stringToIntegerArray(string, array) {
      const alphabet = "abcdefghijklmnopqrstuvwxyz";
      for (let i = 0; i < string.length; i++) {
        array[i] = alphabet.indexOf(string[i]);
      }
    };

    const decode = function integerArrayToString(array) {
      const alphabet = "abcdefghijklmnopqrstuvwxyz";
      let string = "";
      for (let i = 0; i < array.length; i++) {
        string += alphabet[array[i]];
      }
      return string;
    };

    const plaintext = "helloworld";
    const myKey = 3;
    const myArray = new Int32Array(memory.buffer, 0, plaintext.length);

    encode(plaintext, myArray);
    console.log(myArray); // Int32Array(10) [7, 4, 11, 11, 14, 22, 14, 17, 11, 3]
    console.log(decode(myArray)); // helloworld

    caesarEncrypt(myArray.byteOffset, myArray.length, myKey);
    console.log(myArray); // Int32Array(10) [10, 7, 14, 14, 17, 25, 17, 20, 14, 6]
    console.log(decode(myArray)); // khoorzruog

    caesarDecrypt(myArray.byteOffset, myArray.length, myKey);
    console.log(myArray); // Int32Array(10) [7, 4, 11, 11, 14, 22, 14, 17, 11, 3]
    console.log(decode(myArray)); // helloworld
})();
