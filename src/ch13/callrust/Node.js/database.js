// 需要Node.js V10.7.0, N-API已稳定
// 命令：node database.js
const ffi = require('ffi-napi');
const lib = ffi.Library('../target/debug/libcallrust.dylib', {
    database_new: ['pointer', []],
    database_free: ['void', ['pointer']],
    database_insert: ['void', ['pointer']],
    database_query: ['uint32', ['pointer', 'string']],
});
const Database = function() {
    this.ptr = lib.database_new();
};
Database.prototype.free = function() {
   lib.database_free(this.ptr);
};
Database.prototype.insert = function() {
   lib.database_insert(this.ptr);
};
Database.prototype.query = function(zip) {
   return lib.database_query(this.ptr, zip);
};
const database = new Database();
try {
   database.insert();
   const pop1 = database.query("10186")
   const pop2 = database.query("10852")
   console.log(pop2 - pop1);
}finally {
   database.free();
}