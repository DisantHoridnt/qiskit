pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// Import necessary Qiskit modules
use qiskit::quantum_circuit::{QuantumCircuit, TranspiledCircuit}; // Adjust the path based on actual structure

#[no_mangle]
pub extern "C" fn create_quantum_circuit() -> *mut QuantumCircuit {
    let circuit = QuantumCircuit::new();
    Box::into_raw(Box::new(circuit))
}

#[no_mangle]
pub extern "C" fn add_hadamard(circuit: *mut QuantumCircuit, qubit: usize) -> i32 {
    if circuit.is_null() {
        return -1; // Handle null pointer
    }
    let circuit = unsafe { &mut *circuit };
    if qubit >= circuit.num_qubits() {
        return -2; // Handle invalid qubit index
    }
    circuit.h(qubit);
    0
}

#[no_mangle]
pub extern "C" fn add_pauli_x(circuit: *mut QuantumCircuit, qubit: usize) -> i32 {
    if circuit.is_null() {
        return -1; // Handle null pointer
    }
    let circuit = unsafe { &mut *circuit };
    if qubit >= circuit.num_qubits() {
        return -2; // Handle invalid qubit index
    }
    circuit.x(qubit);
    0
}

#[no_mangle]
pub extern "C" fn add_pauli_y(circuit: *mut QuantumCircuit, qubit: usize) -> i32 {
    if circuit.is_null() {
        return -1; // Handle null pointer
    }
    let circuit = unsafe { &mut *circuit };
    if qubit >= circuit.num_qubits() {
        return -2; // Handle invalid qubit index
    }
    circuit.y(qubit);
    0
}

#[no_mangle]
pub extern "C" fn add_pauli_z(circuit: *mut QuantumCircuit, qubit: usize) -> i32 {
    if circuit.is_null() {
        return -1; // Handle null pointer
    }
    let circuit = unsafe { &mut *circuit };
    if qubit >= circuit.num_qubits() {
        return -2; // Handle invalid qubit index
    }
    circuit.z(qubit);
    0
}

#[no_mangle]
pub extern "C" fn add_t(circuit: *mut QuantumCircuit, qubit: usize) -> i32 {
    if circuit.is_null() {
        return -1; // Handle null pointer
    }
    let circuit = unsafe { &mut *circuit };
    if qubit >= circuit.num_qubits() {
        return -2; // Handle invalid qubit index
    }
    circuit.t(qubit);
    0
}

#[no_mangle]
pub extern "C" fn add_s(circuit: *mut QuantumCircuit, qubit: usize) -> i32 {
    if circuit.is_null() {
        return -1; // Handle null pointer
    }
    let circuit = unsafe { &mut *circuit };
    if qubit >= circuit.num_qubits() {
        return -2; // Handle invalid qubit index
    }
    circuit.s(qubit);
    0
}

#[no_mangle]
pub extern "C" fn add_cnot(circuit: *mut QuantumCircuit, control: usize, target: usize) -> i32 {
    if circuit.is_null() {
        return -1; // Handle null pointer
    }
    let circuit = unsafe { &mut *circuit };
    if control >= circuit.num_qubits() || target >= circuit.num_qubits() {
        return -2; // Handle invalid qubit index
    }
    circuit.cx(control, target);
    0
}

#[no_mangle]
pub extern "C" fn transpile_circuit(circuit: *mut QuantumCircuit) -> *mut TranspiledCircuit {
    if circuit.is_null() {
        return std::ptr::null_mut(); // Handle null pointer
    }
    let circuit = unsafe { Box::from_raw(circuit) };
    let transpiled = circuit.transpile();
    Box::into_raw(Box::new(transpiled))
}

#[no_mangle]
pub extern "C" fn free_quantum_circuit(circuit: *mut QuantumCircuit) {
    if circuit.is_null() { return; }
    unsafe { Box::from_raw(circuit); } // Free the memory
}

#[no_mangle]
pub extern "C" fn free_transpiled_circuit(transpiled: *mut TranspiledCircuit) {
    if transpiled.is_null() { return; }
    unsafe { Box::from_raw(transpiled); } // Free the memory
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
