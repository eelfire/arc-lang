(module
  (type (;0;) (func (param i32 i32 i32)))
  (type (;1;) (func (result i32)))
  (type (;2;) (func))
  (type (;3;) (func (param i32)))
  (type (;4;) (func (param i32) (result i32)))
  (func (;0;) (type 2)
    nop
  )
  (func (;1;) (type 0) (param i32 i32 i32)
    (local i32 i32)
    local.get 1
    i32.const 0
    i32.gt_s
    if ;; label = @1
      loop ;; label = @2
        local.get 0
        local.get 3
        i32.const 2
        i32.shl
        i32.add
        local.tee 4
        local.get 4
        i32.load
        local.get 2
        i32.add
        i32.const 26
        i32.rem_s
        i32.store
        local.get 3
        i32.const 1
        i32.add
        local.tee 3
        local.get 1
        i32.ne
        br_if 0 (;@2;)
      end
    end
  )
  (func (;2;) (type 0) (param i32 i32 i32)
    (local i32 i32)
    local.get 1
    i32.const 0
    i32.gt_s
    if ;; label = @1
      loop ;; label = @2
        local.get 0
        local.get 3
        i32.const 2
        i32.shl
        i32.add
        local.tee 4
        local.get 4
        i32.load
        local.get 2
        i32.sub
        i32.const 26
        i32.rem_s
        i32.store
        local.get 3
        i32.const 1
        i32.add
        local.tee 3
        local.get 1
        i32.ne
        br_if 0 (;@2;)
      end
    end
  )
  (func (;3;) (type 1) (result i32)
    global.get 0
  )
  (func (;4;) (type 3) (param i32)
    local.get 0
    global.set 0
  )
  (func (;5;) (type 4) (param i32) (result i32)
    global.get 0
    local.get 0
    i32.sub
    i32.const -16
    i32.and
    local.tee 0
    global.set 0
    local.get 0
  )
  (func (;6;) (type 1) (result i32)
    i32.const 1024
  )
  (table (;0;) 2 2 funcref)
  (memory (;0;) 256 256)
  (global (;0;) (mut i32) i32.const 5243920)
  (export "memory" (memory 0))
  (export "caesarEncrypt" (func 1))
  (export "caesarDecrypt" (func 2))
  (export "_initialize" (func 0))
  (export "__indirect_function_table" (table 0))
  (export "__errno_location" (func 6))
  (export "stackSave" (func 3))
  (export "stackRestore" (func 4))
  (export "stackAlloc" (func 5))
  (elem (;0;) (i32.const 1) func 0)
)