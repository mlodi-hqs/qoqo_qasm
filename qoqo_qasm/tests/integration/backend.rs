// Copyright © 2022 HQS Quantum Simulations GmbH. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
// in compliance with the License. You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed under the
// License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
// express or implied. See the License for the specific language governing permissions and
// limitations under the License.
//
//! Testing the qoqo-qasm Backend

use pyo3::exceptions::PyTypeError;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use qoqo::QoqoBackendError;
use roqoqo::RoqoqoBackendError;

use std::env::temp_dir;
use std::fs;
use std::path::Path;

use qoqo_calculator::CalculatorFloat;

use qoqo_qasm::QasmBackendWrapper;

use qoqo::operations::convert_operation_to_pyobject;
use qoqo::CircuitWrapper;

use roqoqo::operations::*;
use roqoqo::Circuit;

use test_case::test_case;

// helper functions
fn circuitpy_from_circuitru(py: Python, circuit: Circuit) -> &PyCell<CircuitWrapper> {
    let circuit_type = py.get_type::<CircuitWrapper>();
    let circuitpy = circuit_type
        .call0()
        .unwrap()
        .cast_as::<PyCell<CircuitWrapper>>()
        .unwrap();
    for op in circuit {
        let new_op = convert_operation_to_pyobject(op).unwrap();
        circuitpy.call_method1("add", (new_op.clone(),)).unwrap();
    }
    circuitpy
}

fn new_qasmbackend(py: Python, qubit_register_name: Option<String>) -> &PyCell<QasmBackendWrapper> {
    let circuit_type = py.get_type::<QasmBackendWrapper>();
    circuit_type
        .call1((qubit_register_name,))
        .unwrap()
        .cast_as::<PyCell<QasmBackendWrapper>>()
        .unwrap()
}

/// Test circuit_to_qasm_str on a simple Circuit
#[test]
fn test_circuit_to_qasm_str() {
    let mut circuit = Circuit::new();
    circuit += DefinitionBit::new("ro".to_string(), 2, true);
    circuit += RotateX::new(0, std::f64::consts::FRAC_PI_2.into());
    circuit += PauliX::new(1);
    circuit += PragmaRepeatedMeasurement::new("ro".to_string(), 20, None);

    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let backendpy = new_qasmbackend(py, None);
        let circuitpy = circuitpy_from_circuitru(py, circuit);

        let result: String = backendpy
            .call_method1("circuit_to_qasm_str", (circuitpy,))
            .unwrap()
            .extract()
            .unwrap();
        let lines = String::from("OPENQASM 2.0;\ninclude \"qelib1.inc\";\n\nqreg q[2];\ncreg ro[2];\nrx(1.5707963267948966) q[0];\nx q[1];\nmeasure q -> ro;\n");
        assert_eq!(lines, result);
    })
}

/// Test circuit_to_qasm_file on a simple Circuit
#[test]
fn test_circuit_to_qasm_file() {
    let mut circuit = Circuit::new();
    circuit += DefinitionBit::new("ro".to_string(), 2, true);
    circuit += RotateX::new(0, std::f64::consts::FRAC_PI_2.into());
    circuit += PauliX::new(1);
    circuit += PragmaRepeatedMeasurement::new("ro".to_string(), 20, None);

    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let backendpy = new_qasmbackend(py, Some("qr".to_string()));
        let circuitpy = circuitpy_from_circuitru(py, circuit);

        backendpy
            .call_method1(
                "circuit_to_qasm_file",
                (circuitpy, temp_dir().to_str().unwrap(), "fnametest", true),
            )
            .unwrap();

        let lines = String::from("OPENQASM 2.0;\ninclude \"qelib1.inc\";\n\nqreg qr[2];\ncreg ro[2];\nrx(1.5707963267948966) qr[0];\nx qr[1];\nmeasure qr -> ro;\n");
        let read_in_path = temp_dir().join(Path::new("fnametest.qasm"));
        let extracted = fs::read_to_string(&read_in_path);
        fs::remove_file(&read_in_path).unwrap();
        assert_eq!(lines, extracted.unwrap());
    })
}

/// Test circuit_to_qasm_str and circuit_to_qasm_file errors
#[test_case(Operation::from(ISwap::new(0, 1)))]
#[test_case(Operation::from(ControlledPhaseShift::new(0, 1, CalculatorFloat::from(0.23))))]
#[test_case(Operation::from(FSwap::new(0, 1)))]
#[test_case(Operation::from(RotateXY::new(
    0,
    CalculatorFloat::from(0.23),
    CalculatorFloat::from(0.23)
)))]
fn test_circuit_to_qasm_error(operation: Operation) {
    let mut wrong_circuit = Circuit::new();
    wrong_circuit += operation.clone();

    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let wrongcircuitpy = circuitpy_from_circuitru(py, wrong_circuit.clone());

        let backendpy = new_qasmbackend(py, None);
        let result = backendpy.call_method1("circuit_to_qasm_str", (3,));
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            PyTypeError::new_err(format!(
                "Cannot convert python object to Circuit: {:?}",
                QoqoBackendError::CannotExtractObject
            ))
            .to_string()
        );

        let result = backendpy.call_method1("circuit_to_qasm_str", (wrongcircuitpy,));
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            PyValueError::new_err(format!(
                "Error during QASM translation: {:?}",
                RoqoqoBackendError::OperationNotInBackend {
                    backend: "QASM",
                    hqslang: operation.hqslang(),
                }
            ))
            .to_string()
        );

        let backendpy = new_qasmbackend(py, None);
        let result = backendpy.call_method1(
            "circuit_to_qasm_file",
            (3, temp_dir().to_str().unwrap(), "fnametest", true),
        );
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            PyTypeError::new_err(format!(
                "Cannot convert python object to Circuit: {:?}",
                QoqoBackendError::CannotExtractObject
            ))
            .to_string()
        );

        let result = backendpy.call_method1(
            "circuit_to_qasm_file",
            (
                wrongcircuitpy,
                temp_dir().to_str().unwrap(),
                "fnametest",
                true,
            ),
        );
        assert_eq!(
            result.unwrap_err().to_string(),
            PyValueError::new_err(format!(
                "Error during QASM translation: {:?}",
                RoqoqoBackendError::OperationNotInBackend {
                    backend: "QASM",
                    hqslang: operation.hqslang(),
                }
            ))
            .to_string()
        );
    })
}