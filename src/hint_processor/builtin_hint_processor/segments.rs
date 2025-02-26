use crate::hint_processor::{
    builtin_hint_processor::hint_utils::insert_value_from_var_name,
    hint_processor_definition::HintReference,
};

use crate::serde::deserialize_program::ApTracking;

use crate::vm::errors::vm_errors::VirtualMachineError;
use crate::vm::vm_core::VirtualMachine;

use std::collections::HashMap;

use crate::hint_processor::builtin_hint_processor::hint_utils::get_ptr_from_var_name;

/*
Implements hint:
%{ memory.add_relocation_rule(src_ptr=ids.src_ptr, dest_ptr=ids.dest_ptr) %}
*/
pub fn relocate_segment(
    vm: &mut VirtualMachine,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
) -> Result<(), VirtualMachineError> {
    let src_ptr = get_ptr_from_var_name("src_ptr", vm, ids_data, ap_tracking)?;
    let dest_ptr = get_ptr_from_var_name("dest_ptr", vm, ids_data, ap_tracking)?;

    vm.add_relocation_rule(src_ptr, dest_ptr)?;
    Ok(())
}

/*
This hint doesn't belong to the Cairo common library
It's only added for testing proposes

Implements hint:
%{ ids.temporary_array = segments.add_temp_segment() %}
*/
pub fn temporary_array(
    vm: &mut VirtualMachine,
    ids_data: &HashMap<String, HintReference>,
    ap_tracking: &ApTracking,
) -> Result<(), VirtualMachineError> {
    let temp_segment = vm.add_temporary_segment();
    insert_value_from_var_name("temporary_array", temp_segment, vm, ids_data, ap_tracking)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::any_box;
    use crate::bigint;
    use crate::hint_processor::builtin_hint_processor::builtin_hint_processor_definition::BuiltinHintProcessor;
    use crate::hint_processor::builtin_hint_processor::builtin_hint_processor_definition::HintProcessorData;
    use crate::hint_processor::builtin_hint_processor::hint_code;
    use crate::hint_processor::hint_processor_definition::HintProcessor;
    use crate::types::exec_scope::ExecutionScopes;
    use crate::types::relocatable::MaybeRelocatable;
    use crate::utils::test_utils::*;
    use crate::vm::errors::memory_errors::MemoryError;
    use crate::vm::vm_core::VirtualMachine;
    use crate::vm::vm_memory::memory::Memory;
    use num_bigint::BigInt;
    use num_bigint::Sign;
    use std::any::Any;

    #[test]
    fn run_relocate_segment() {
        let hint_code = hint_code::RELOCATE_SEGMENT;
        //Initialize vm
        let mut vm = vm!();
        //Initialize fp
        vm.run_context.fp = 2;
        //Insert ids into memory
        vm.memory = memory![((1, 0), (-2, 0)), ((1, 1), (3, 0)), ((3, 0), 42)];

        //Create ids_data & hint_data
        let ids_data = ids_data!["src_ptr", "dest_ptr"];

        //Execute the hint
        assert_eq!(run_hint!(vm, ids_data, hint_code), Ok(()));

        vm.memory
            .relocate_memory()
            .expect("Couldn't relocate memory.");
    }

    #[test]
    fn run_temporary_array() {
        let hint_code = hint_code::TEMPORARY_ARRAY;
        //Initialize vm
        let mut vm = vm!();
        vm.segments.add(&mut vm.memory);
        vm.segments.add(&mut vm.memory);
        //Initialize fp
        vm.run_context.fp = 1;

        //Create ids_data & hint_data
        let ids_data = ids_data!["temporary_array"];

        //Execute the hint
        assert_eq!(run_hint!(vm, ids_data, hint_code), Ok(()));
        check_memory!(vm.memory, ((1, 0), (-1, 0)));
    }
}
