(module
  (type (;0;) (func (param i32 i32) (result i32)))
  (type (;1;) (func (result i32)))
  (type (;2;) (func))
  (type (;3;) (func (param i32)))
  (type (;4;) (func (param i32) (result i32)))
  (func (;0;) (type 2)
    nop
  )
  (func (;1;) (type 0) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.add
  )
  (func (;2;) (type 0) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.sub
  )
  (func (;3;) (type 0) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.mul
  )
  (func (;4;) (type 0) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.div_s
  )
  (func (;5;) (type 0) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.rem_s
  )
  (func (;6;) (type 1) (result i32)
    global.get 0
  )
  (func (;7;) (type 3) (param i32)
    local.get 0
    global.set 0
  )
  (func (;8;) (type 4) (param i32) (result i32)
    global.get 0
    local.get 0
    i32.sub
    i32.const -16
    i32.and
    local.tee 0
    global.set 0
    local.get 0
  )
  (func (;9;) (type 1) (result i32)
    i32.const 1024
  )
  (table (;0;) 2 2 funcref)
  (memory (;0;) 256 256)
  (global (;0;) (mut i32) i32.const 5243920)
  (export "memory" (memory 0))
  (export "add" (func 1))
  (export "sub" (func 2))
  (export "mul" (func 3))
  (export "div" (func 4))
  (export "mod" (func 5))
  (export "_initialize" (func 0))
  (export "__indirect_function_table" (table 0))
  (export "__errno_location" (func 9))
  (export "stackSave" (func 6))
  (export "stackRestore" (func 7))
  (export "stackAlloc" (func 8))
  (elem (;0;) (i32.const 1) func 0)
)