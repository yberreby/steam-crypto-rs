var crypto = require('crypto');
var constants = require('constants');
var publicKey = require('fs').readFileSync(__dirname + '/../steam.pub');

exports.generateSessionKey = function() {
  var sessionKey = crypto.randomBytes(32);
  var cryptedSessionKey = crypto.publicEncrypt(publicKey, sessionKey);
  return {
    plain: sessionKey,
    encrypted: cryptedSessionKey
  };
};

exports.symmetricEncrypt = function(input, key) {
  var iv = crypto.randomBytes(16);

  // we create a Cipher which is a readable and writable stream
  // ECB has no initialization vector. Plaintext block -> encrypted block
  var aesIv = crypto.createCipheriv('aes-256-ecb', key, '');
  // no padding because input is... a multiple of the block size?
  // ah yes. The input is the plaintext, random IV. Which is 16 bytes.
  aesIv.setAutoPadding(false);
  // we write the random IV to it
  aesIv.end(iv);

  // create a new Cipher
  // CBC = Chained Block Cipher
  // the iv is what we generated earlier
  var aesData = crypto.createCipheriv('aes-256-cbc', key, iv);
  aesData.end(input);

  return Buffer.concat([aesIv.read(), aesData.read()]);

  // to decrypt the data, Steam will need
};

exports.symmetricDecrypt = function(input, key) {
  var aesIv = crypto.createDecipheriv('aes-256-ecb', key, '');
  aesIv.setAutoPadding(false);
  aesIv.end(input.slice(0, 16));

  var aesData = crypto.createDecipheriv('aes-256-cbc', key, aesIv.read());
  aesData.end(input.slice(16));

  return aesData.read();
};

function symmetricEncryptWithIv(input, key, iv) {
  var aesIv = crypto.createCipheriv('aes-256-ecb', key, '');
  aesIv.setAutoPadding(false);
  aesIv.end(iv);

  var aesData = crypto.createCipheriv('aes-256-cbc', key, iv);
  aesData.end(input);

  return Buffer.concat([aesIv.read(), aesData.read()]);
}

var iv = new Buffer("7b2d7972d6eb4142edc26206b224089a", 'hex')
var key = new Buffer('d10f33200d2372699513478275400073715478ac3794339c8b526433479767de', 'hex')
console.log(
  symmetricEncryptWithIv('The quick brown fox jumped over the lazy dog.', key, iv).toString('hex')
)
