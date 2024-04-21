(module
  (type (;0;) (func (result i32)))
  (type (;1;) (func))
  (type (;2;) (func (param i32 i32)))
  (type (;3;) (func (param i32)))
  (type (;4;) (func (param i32) (result i32)))
  (func (;0;) (type 1)
    nop
  )
  (func (;1;) (type 2) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32)
    local.get 1
    i32.const 0
    i32.gt_s
    if ;; label = @1
      local.get 1
      local.set 3
      loop ;; label = @2
        local.get 3
        i32.const 1
        i32.sub
        local.set 3
        local.get 2
        i32.const -1
        i32.xor
        local.get 1
        i32.add
        i32.const 0
        i32.gt_s
        if ;; label = @3
          local.get 0
          i32.load
          local.set 4
          i32.const 0
          local.set 5
          loop ;; label = @4
            block ;; label = @5
              local.get 0
              local.get 5
              i32.const 1
              i32.add
              local.tee 7
              i32.const 2
              i32.shl
              i32.add
              local.tee 8
              i32.load
              local.tee 6
              local.get 4
              i32.ge_s
              if ;; label = @6
                local.get 6
                local.set 4
                br 1 (;@5;)
              end
              local.get 8
              local.get 4
              i32.store
              local.get 0
              local.get 5
              i32.const 2
              i32.shl
              i32.add
              local.get 6
              i32.store
            end
            local.get 7
            local.tee 5
            local.get 3
            i32.ne
            br_if 0 (;@4;)
          end
        end
        local.get 2
        i32.const 1
        i32.add
        local.tee 2
        local.get 1
        i32.ne
        br_if 0 (;@2;)
      end
    end
  )
  (func (;2;) (type 0) (result i32)
    global.get 0
  )
  (func (;3;) (type 3) (param i32)
    local.get 0
    global.set 0
  )
  (func (;4;) (type 4) (param i32) (result i32)
    global.get 0
    local.get 0
    i32.sub
    i32.const -16
    i32.and
    local.tee 0
    global.set 0
    local.get 0
  )
  (func (;5;) (type 0) (result i32)
    i32.const 1024
  )
  (table (;0;) 2 2 funcref)
  (memory (;0;) 256 256)
  (global (;0;) (mut i32) i32.const 5243920)
  (export "memory" (memory 0))
  (export "sort" (func 1))
  (export "_initialize" (func 0))
  (export "__indirect_function_table" (table 0))
  (export "__errno_location" (func 5))
  (export "stackSave" (func 2))
  (export "stackRestore" (func 3))
  (export "stackAlloc" (func 4))
  (elem (;0;) (i32.const 1) func 0)
)