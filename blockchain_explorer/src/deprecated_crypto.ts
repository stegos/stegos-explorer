import crypto from "crypto-browserify";
import { Buffer } from "buffer";

const algorithm = "aes-128-ctr"; // todo config
const tokenLength = 16;

const encrypt = (plaintext: string, key: string) => {
  const resizedIV = Buffer.allocUnsafe(tokenLength);
  const iv = crypto
    .createHash("sha256")
    .update(Buffer.from(crypto.randomBytes(tokenLength)).toString("hex"))
    .digest();
  iv.copy(resizedIV);
  const encryptor = crypto.createCipheriv(
    algorithm,
    Buffer.from(key, "base64"),
    resizedIV
  );
  return arrayBufferToBase64(
    Buffer.concat([resizedIV, encryptor.update(plaintext), encryptor.final()])
  );
};

const decrypt = (text: string, key: string) => {
  const buffer = base64ToArrayBuffer(text);
  const resizedIV = Buffer.allocUnsafe(tokenLength);
  Buffer.from(buffer, 0, tokenLength).copy(resizedIV);
  const ct = Buffer.from(buffer, tokenLength);
  const decipher = crypto.createDecipheriv(
    algorithm,
    Buffer.from(key, "base64"),
    resizedIV
  );
  const dec = Buffer.concat([decipher.update(ct), decipher.final()]);
  return dec.toString("utf8");
};

const base64ToArrayBuffer = (base64: string) => {
  const binaryString = atob(base64);
  const len = binaryString.length;
  const bytes = new Uint8Array(len);
  for (let i = 0; i < len; i += 1) {
    bytes[i] = binaryString.charCodeAt(i);
  }
  return bytes.buffer;
};

const arrayBufferToBase64 = (buffer: Buffer) => {
  let binary = "";
  const bytes = new Uint8Array(buffer);
  const len = bytes.byteLength;
  for (let i = 0; i < len; i += 1) {
    binary += String.fromCharCode(bytes[i]);
  }
  return btoa(binary);
};

const mycrypto = {
  arrayBufferToBase64,
  base64ToArrayBuffer,
  decrypt,
  encrypt
};

export default mycrypto;
