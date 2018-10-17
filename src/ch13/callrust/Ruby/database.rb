# 命令：ruby database.rb
require 'ffi'
class Database < FFI::AutoPointer
  def self.release(ptr)
    Binding.free(ptr)
  end
  def insert
    Binding.insert(self)
  end
  def query(zip)
   Binding.query(self, zip)
 end
  module Binding
    extend FFI::Library
    # 因为我本地使用macOS，所以这动态库为.dylib
    ffi_lib "../target/debug/libcallrust.dylib"
    attach_function :new, :database_new, [], Database
    attach_function :free, :database_free, [Database], :void
    attach_function :insert, :database_insert, [Database], :void
    attach_function :query, :database_query,
      [Database, :string], :uint32
  end
end
database = Database::Binding.new
database.insert
pop1 = database.query("10186")
pop2 = database.query("10852")
puts pop2 - pop1