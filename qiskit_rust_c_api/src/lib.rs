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
pub extern "C" fn add_hadamard(circuit: *mut QuantumCircuit, qubit: usize) {
    let circuit = unsafe { &mut *circuit };
    circuit.h(qubit);
}

#[no_mangle]
pub extern "C" fn add_pauli_x(circuit: *mut QuantumCircuit, qubit: usize) {
    let circuit = unsafe { &mut *circuit };
    circuit.x(qubit);
}

#[no_mangle]
pub extern "C" fn add_cnot(circuit: *mut QuantumCircuit, control: usize, target: usize) {
    let circuit = unsafe { &mut *circuit };
    circuit.cx(control, target);
}

#[no_mangle]
pub extern "C" fn transpile_circuit(circuit: *mut QuantumCircuit) -> *mut TranspiledCircuit {
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
