# Copyright © 2023 HQS Quantum Simulations GmbH.
"""Test file for conversion.py."""

import pytest
import sys
from qoqo import Circuit
from qoqo import operations as ops
from qiskit import QuantumCircuit, QuantumRegister, ClassicalRegister

from qoqo_qiskit.conversion import to_qiskit_circuit


def test_basic_circuit():
    circuit = Circuit()
    circuit += ops.Hadamard(0)
    circuit += ops.PauliX(1)

    qc = QuantumCircuit(2)
    qc.h(0)
    qc.x(1)

    out_circ, sim_dict = to_qiskit_circuit(circuit)

    assert (out_circ == qc)
    assert (len(sim_dict["measurements"]) == 0)


def test_qreg_creg_names():
    circuit = Circuit()
    circuit += ops.DefinitionBit('cr', 2, is_output=True)
    circuit += ops.DefinitionBit('crr', 3, is_output=True)

    qr = QuantumRegister(1, 'qrg')
    cr = ClassicalRegister(2, 'cr')
    cr2 = ClassicalRegister(3, 'crr')
    qc = QuantumCircuit(qr, cr, cr2)

    out_circ, _ = to_qiskit_circuit(circuit, qubit_register_name='qrg')

    assert (out_circ == qc)


def test_setstatevector():
    circuit = Circuit()
    circuit += ops.PragmaSetStateVector([0, 1])

    qc = QuantumCircuit(1)
    qc.initialize([0, 1])

    out_circ, _ = to_qiskit_circuit(circuit)

    assert (out_circ == qc)

    circuit = Circuit()
    circuit += ops.PragmaSetStateVector([0, 1])
    circuit += ops.RotateX(0, 0.23)

    qc = QuantumCircuit(1)
    qc.initialize([0, 1])
    qc.rx(0.23, 0)

    out_circ, _ = to_qiskit_circuit(circuit)

    assert (out_circ == qc)


def test_repeated_measurement():
    circuit = Circuit()
    circuit += ops.DefinitionBit("ri", 1, True)
    circuit += ops.PragmaRepeatedMeasurement("ri", 300)

    qr = QuantumRegister(1, 'q')
    cr = ClassicalRegister(1, 'ri')
    qc = QuantumCircuit(qr, cr)
    qc.measure(0, cr)

    out_circ, sim_dict = to_qiskit_circuit(circuit)

    assert (out_circ == qc)
    assert (('ri', 300, None) in sim_dict["measurements"])


def test_measure_qubit():
    pass


# For pytest
if __name__ == '__main__':
    pytest.main(sys.argv)
