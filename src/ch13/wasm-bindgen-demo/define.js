export function hello(ptr, len) {
    var buf = new Uint8Array(mod.instance.exports.memory.buffer, ptr, len)
    var msg = new TextDecoder('utf8').decode(buf);
    alert(msg);
  }
  