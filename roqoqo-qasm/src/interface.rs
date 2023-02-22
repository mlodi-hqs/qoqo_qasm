// Copyright © 2021-2023 HQS Quantum Simulations GmbH. All Rights Reserved.
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
//! The roqoqo-qasm Interface translates qoqo operations and circuits to QASM operations via the interface.

use qoqo_calculator::{CalculatorComplex, CalculatorFloat};
use roqoqo::operations::*;
use roqoqo::Circuit;
use roqoqo::RoqoqoBackendError;

// Pragma operations that are ignored by backend and do not throw an error
const ALLOWED_OPERATIONS: &[&str; 7] = &[
    "PragmaSleep",
    "PragmaGlobalPhase",
    "PragmaStopParallelBlock",
    "PragmaStopDecompositionBlock",
    "PragmaSetNumberOfMeasurements",
    "PragmaStartDecompositionBlock",
    "InputSymbolic",
];

/// Translate the qoqo circuit into QASM ouput.
///
/// The qoqo_qasm interface iterates through the qoqo circuit and translates each qoqo operation
/// to QASM output (strings).
///
/// # Arguments
///
/// * `circuit` - The qoqo Circuit that is translated.
/// * `qubit_register_name` - Name of the quantum register used for the roqoqo address-space
///
/// # Returns
///
/// * `Ok(Vec<&str>)` - Vector containing converted operations as strings.
/// * `Err(RoqoqoBackendError)` - Operation not supported by QASM backend.
///
/// # Example
/// ```
/// use roqoqo::{Circuit, operations::{DefinitionBit, PauliX, MeasureQubit}};
/// use roqoqo_qasm::call_circuit;
/// use std::collections::HashMap;
///
/// let mut circuit = Circuit::new();
/// circuit += DefinitionBit::new("ro".to_string(), 1, true);
/// circuit += PauliX::new(0);
/// circuit += MeasureQubit::new(0, "ro".to_string(), 0);
/// let circuit: Vec<String> = call_circuit(&circuit, "q").unwrap();
///
/// let manual_circuit: Vec<String> = vec![
///     "creg ro[1];".to_string(),
///     "x q[0];".to_string(),
///     "measure q[0] -> ro[0];".to_string()
/// ];
///
/// assert_eq!(circuit, manual_circuit);
/// ```
///
pub fn call_circuit(
    circuit: &Circuit,
    qubit_register_name: &str,
) -> Result<Vec<String>, RoqoqoBackendError> {
    let mut str_circuit: Vec<String> = Vec::new();
    for op in circuit.iter() {
        str_circuit.push(call_operation(op, qubit_register_name)?);
    }
    Ok(str_circuit)
}

/// Translates a qoqo operation to QASM (&str).
///
/// # Arguments
///
/// * `operation` - The qoqo Operation that is executed.
///
/// # Returns
///
/// * `Ok(&str)` - Converted operation in &str form.
/// * `Err(RoqoqoBackendError)` - Operation not supported by QASM backend.
///
pub fn call_operation(
    operation: &Operation,
    qubit_register_name: &str,
) -> Result<String, RoqoqoBackendError> {
    match operation {
        Operation::RotateZ(op) => Ok(format!(
            "rz({}) {}[{}];",
            op.theta().float().unwrap(),
            qubit_register_name,
            op.qubit()
        )),
        Operation::RotateX(op) => Ok(format!(
            "rx({}) {}[{}];",
            op.theta().float().unwrap(),
            qubit_register_name,
            op.qubit()
        )),
        Operation::RotateY(op) => Ok(format!(
            "ry({}) {}[{}];",
            op.theta().float().unwrap(),
            qubit_register_name,
            op.qubit()
        )),
        Operation::Hadamard(op) => Ok(format!("h {}[{}];", qubit_register_name, op.qubit())),
        Operation::PauliX(op) => Ok(format!("x {}[{}];", qubit_register_name, op.qubit())),
        Operation::PauliY(op) => Ok(format!("y {}[{}];", qubit_register_name, op.qubit())),
        Operation::PauliZ(op) => Ok(format!("z {}[{}];", qubit_register_name, op.qubit())),
        Operation::SGate(op) => Ok(format!("s {}[{}];", qubit_register_name, op.qubit())),
        Operation::TGate(op) => Ok(format!("t {}[{}];", qubit_register_name, op.qubit())),
        Operation::PhaseShiftState1(op) => Ok(format!(
            "p({}) {}[{}];",
            op.theta().float().unwrap(),
            qubit_register_name,
            op.qubit()
        )),
        Operation::SqrtPauliX(op) => Ok(format!("sx {}[{}];", qubit_register_name, op.qubit())),
        Operation::InvSqrtPauliX(op) => {
            Ok(format!("sxdg {}[{}];", qubit_register_name, op.qubit()))
        }
        Operation::CNOT(op) => Ok(format!(
            "cx {}[{}],{}[{}];",
            qubit_register_name,
            op.control(),
            qubit_register_name,
            op.target()
        )),
        Operation::MolmerSorensenXX(op) => Ok(format!(
            "rxx(pi/2) {}[{}],{}[{}];",
            qubit_register_name,
            op.control(),
            qubit_register_name,
            op.target()
        )),
        Operation::VariableMSXX(op) => Ok(format!(
            "rxx({}) {}[{}],{}[{}];",
            op.theta(),
            qubit_register_name,
            op.control(),
            qubit_register_name,
            op.target()
        )),
        Operation::ControlledPauliY(op) => Ok(format!(
            "cy {}[{}],{}[{}];",
            qubit_register_name,
            op.control(),
            qubit_register_name,
            op.target()
        )),
        Operation::ControlledPauliZ(op) => Ok(format!(
            "cz {}[{}],{}[{}];",
            qubit_register_name,
            op.control(),
            qubit_register_name,
            op.target()
        )),
        Operation::ControlledPhaseShift(op) => Ok(format!(
            "cp({}) {}[{}],{}[{}];",
            op.theta(),
            qubit_register_name,
            op.control(),
            qubit_register_name,
            op.target()
        )),
        Operation::SWAP(op) => Ok(format!(
            "swap {}[{}],{}[{}];",
            qubit_register_name,
            op.control(),
            qubit_register_name,
            op.target()
        )),
        Operation::SingleQubitGate(op) => {
            let alpha = CalculatorComplex::new(op.alpha_r(), op.alpha_i());
            let beta = CalculatorComplex::new(op.beta_r(), op.beta_i());
            let theta: CalculatorFloat = alpha.norm().acos() * 2.0;
            let phi: CalculatorFloat = alpha.arg() * (-1.0) + beta.arg();
            let lamda: CalculatorFloat = alpha.arg() * (-1.0) - beta.arg();

            Ok(format!(
                "u3({:.15},{:.15},{:.15}) {}[{}];",
                theta.float().unwrap(),
                phi.float().unwrap(),
                lamda.float().unwrap(),
                qubit_register_name,
                op.qubit()
            ))
        }
        Operation::PragmaActiveReset(op) => {
            Ok(format!("reset {}[{}];", qubit_register_name, op.qubit(),))
        }
        Operation::PragmaConditional(op) => {
            // can't handle multiple operations under if condition
            let mut ite = op.circuit().iter().peekable();
            let mut data = "".to_string();
            while let Some(int_op) = ite.next() {
                if ite.peek().is_none() {
                    data.push_str(&format!(
                        "if({}[{}]==1) {}",
                        op.condition_register(),
                        op.condition_index(),
                        call_operation(int_op, qubit_register_name).unwrap()
                    ));
                } else {
                    data.push_str(&format!(
                        "if({}[{}]==1) {}\n",
                        op.condition_register(),
                        op.condition_index(),
                        call_operation(int_op, qubit_register_name).unwrap()
                    ));
                }
            }
            Ok(data)
        }
        Operation::PragmaRepeatedMeasurement(op) => match op.qubit_mapping() {
            None => Ok(format!(
                "measure {} -> {};",
                qubit_register_name,
                op.readout()
            )),
            Some(qm) => {
                let mut output_string = "".to_string();
                for (key, val) in qm.iter() {
                    output_string += format!(
                        "measure {}[{}] -> {}[{}];\n",
                        qubit_register_name,
                        key,
                        op.readout(),
                        val
                    )
                    .as_str();
                }
                Ok(output_string)
            }
        },
        Operation::MeasureQubit(op) => Ok(format!(
            "measure {}[{}] -> {}[{}];",
            qubit_register_name,
            op.qubit(),
            op.readout(),
            op.readout_index()
        )),
        Operation::DefinitionFloat(op) => Ok(format!("creg {}[{}];", op.name(), op.length())),
        Operation::DefinitionUsize(op) => Ok(format!("creg {}[{}];", op.name(), op.length())),
        Operation::DefinitionBit(op) => Ok(format!("creg {}[{}];", op.name(), op.length())),
        Operation::DefinitionComplex(op) => Ok(format!("creg {}[{}];", op.name(), op.length())),
        _ => {
            if ALLOWED_OPERATIONS.contains(&operation.hqslang()) {
                Ok("".to_string())
            } else {
                Err(RoqoqoBackendError::OperationNotInBackend {
                    backend: "QASM",
                    hqslang: operation.hqslang(),
                })
            }
        }
    }
}

/// Outputs the QASM gate definition of many qoqo operations.
///
/// # Arguments:
///
/// * `operation` - The roqoqo Operation to be defined.
///
/// # Returns:
///
/// * `Ok(String)` - The gate QASM gate definition.
/// * `RoqoqoBackendError::GenericError` - TODO
pub fn gate_definition(operation: &Operation) -> Result<String, RoqoqoBackendError> {
    match operation {
        // TODO: add u1 u2 u3 definitions
        // TODO: add all basic definitions of the ops up
        Operation::RotateX(_) => Ok(String::from(
            "gate rx(theta) a { u3(theta, -pi/2,pi/2) a; }\n"
        )),
        Operation::RotateY(_) => Ok(String::from(
            "gate ry(theta) a { u3(theta,0,0) a; }\n"
        )),
        Operation::RotateZ(_) => Ok(String::from(
            "gate rz(phi) a { u1(phi) a; }\n"
        )),
        Operation::Hadamard(_) => Ok(String::from(
            "gate h a { u2(0,pi) a; }\n"
        )),
        Operation::CNOT(_) => Ok(String::from(
            "gate cx c,t { CX c,t; }\n"
        )),
        Operation::ISwap(_) => Ok(String::from(
            "gate iswap a,b { rx(pi/2) a; CX a,b; rx(-pi/2) a; ry(-pi/2) b; CX a,b; rx(-pi/2) a; }\n"
        )),
        Operation::SqrtISwap(_) => Ok(String::from(
            "gate siswap a,b { rx(pi/2) a; CX a,b; rx(-pi/4) a; ry(-pi/4) b; CX a,b; rx(-pi/2) a; }\n"
        )),
        Operation::InvSqrtISwap(_) => Ok(String::from(
            "gate siswapdg a,b { rx(pi/2) a; CX a,b; rx(pi/4) a; ry(pi/4) b; CX a,b; rx(-pi/2) a; }\n"
        )),
        Operation::FSwap(_) => Ok(String::from(
            "gate fswap a,b { rz(-pi/2) a; rz(-pi/2) b; rx(pi/2) a; CX a,b; rx(-pi/2) a; ry(-pi/2) b; CX a,b; rx(-pi/2) a; }\n"
        )),
        Operation::Fsim(_) => Ok(String::from(
            "gate fsim(t,u,phi) a,b { rz(-pi/2) a; rz(pi) b; ry(pi/2) b; h b; CX a,b; h b; ry(-t+phi+pi/2) a; rx(pi) a; ry(-pi/2) b; rz((u-pi)/2) b; h b; CX a,b; h b; rz(pi) a; ry(t+phi+pi/2) a; rz(pi) b; ry(pi/2) b; h b; CX a,b; h b; rz(-pi/2) b; rx(-pi/2) b; rz((-u-pi)/2) a; rz((-u-pi)/2) b; }\n"
        )),
        Operation::PMInteraction(_) => Ok(String::from(
            "gate pmint(theta) a,b { rx(pi/2) a; CX a,b; rx(theta) a; ry(theta) b; CX a,b; rx(-pi/2) a; }\n"
        )),
        Operation::GivensRotation(_) => Ok(String::from(
            "gate gvnsrot(theta,phi) a,b { rz(phi+pi/2) b; rx(pi/2) a; CX a,b; rx(-theta) a; ry(-theta) b; CX a,b; rx(-pi/2) a; rz(-pi/2) b; }\n"
        )),
        Operation::GivensRotationLittleEndian(_) => Ok(String::from(
            "gate gvnsrotle(theta,phi) a,b { rz(-pi/2) a; rx(pi/2) a; CX a,b; rx(-theta) a; ry(-theta) b; CX a,b; rx(-pi/2) a; rz(phi+pi/2) a; }\n"
        )),
        Operation::Qsim(_) => Ok(String::from(
            "gate qsim(xc,yc,zc) a,b { rz(-pi/2) a; rz(pi) b; ry(pi/2) b; h b; CX a,b; h b; ry(-2*xc+pi/2) a; rx(pi) a; ry(-pi/2) b; rz(2*zc-pi) b; h b; CX a,b; h b; rz(pi) a; ry(2*yc+pi/2) a; rz(pi) b; ry(pi/2) b; h b; CX a,b; h b; rz(-pi/2) b; rx(-pi/2) b; }\n"
        )),
        Operation::XY(_) => Ok(String::from(
            "gate xy(theta) a,b { rx(pi/2) a; CX a,b; rx(-theta/2) a; ry(-theta/2) b; CX a,b; rx(-pi/2) a; }\n"
        )),
        Operation::SpinInteraction(_) => Ok(String::from(
            "gate spinint(xc,yc,zc) a,b { rz(-pi/2) a; rz(pi) b; ry(pi/2) b; h b; CX a,b; h b; ry(-2*xc) a; rx(pi) a; ry(-pi/2) b; rz(2*zc-pi/2) b; h b; CX a,b; h b; rz(pi) a; ry(2*yc+pi) a; rz(pi) b; ry(pi/2) b; h b; CX a,b; h b; rz(-pi/2) b; rx(-pi/2) b; }\n"
        )),
        Operation::PhaseShiftedControlledZ(_) => Ok(String::from(
            "gate pscz(phi) a,b { rz(pi/2) a; rz(pi/2) b; ry(pi/2) b; CX a,b; rx(-pi/2) b; rz(-pi/2) a; ry(-pi/2) b; rz(phi) a; rz(phi) b; }\n"
        )),
        Operation::PhaseShiftedControlledPhase(_) => Ok(String::from(
            "gate pscp(theta,phi) a,b { rz(theta/2) a; rz(theta/2) b; CX a,b; rz(-theta/2) b; CX a,b; rz(phi) a; rz(phi) b; }\n"
        )),
        // TODO: to be applied to each qubit
        Operation::PragmaGlobalPhase(_) => Ok(String::from(
            "gate gphase(theta) q { x q; u1(theta) q; x q; u1(theta) q; }\n"
        )),
        Operation::RotateXY(_) => Ok(String::from(
            "gate rxy(theta,phi) q { u3(theta,phi-pi/2,pi/2-phi) q; }\n"
        )),
        _ => Err(RoqoqoBackendError::GenericError { msg: "TODO".to_string() }),
    }
}
