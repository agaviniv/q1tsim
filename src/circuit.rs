extern crate ndarray;

use gates;
use qustate;

/// A single operation in a circuit
enum CircuitOp
{
    /// Apply a gate to the state
    Gate(Box<gates::Gate>, Vec<usize>),
    /// Measure a qubit
    Measure(usize, usize)
}

/// A quantum circuit
///
/// Struct Circuit represents a quantum circuit, holding a quantum state and the
/// operations to be performed on it.
pub struct Circuit
{
    /// The quantum state of the system
    q_state: qustate::QuState,
    /// The classial state of the system
    c_state: ndarray::Array2<u8>,
    /// The operations to perform on the state
    ops: Vec<CircuitOp>
}

impl Circuit
{
    /// Create a new circuit.
    ///
    /// Create a new (empty) quantum circuit, with `nr_qubits` quantum bits and
    /// `nr_cbits` classical bits, to be run `nr_shots` times.
    pub fn new(nr_qubits: usize, nr_cbits: usize, nr_shots: usize) -> Self
    {
        Circuit
        {
            q_state: qustate::QuState::new(nr_qubits, nr_shots),
            c_state: ndarray::Array::zeros((nr_cbits, nr_shots)),
            ops: vec![]
        }
    }

    /// The classical register.
    ///
    /// Return a reference to the classical bit register, containing the results
    /// of any measurements made on the system.
    pub fn cstate(&self) -> &ndarray::Array2<u8>
    {
        &self.c_state
    }

    /// Add a gate.
    ///
    /// Append a `n`-ary gate `gate`, operating on the `n` qubits in `bits`, to
    /// this circuit.
    pub fn add_gate<G: 'static>(&mut self, gate: G, bits: &[usize])
    where G: gates::Gate
    {
        self.ops.push(CircuitOp::Gate(Box::new(gate), bits.to_owned()));
    }

    /// Add a measurement.
    ///
    /// Add measurement of qubit `qbit` into classical bit `cbit` to this circuit.
    pub fn add_measurement(&mut self, qbit: usize, cbit: usize)
    {
        self.ops.push(CircuitOp::Measure(qbit, cbit));
    }

    /// Execute this circuit
    ///
    /// Execute this circuit, performing its operations and measurements. Note
    /// that this does not reset the state before execution. In case multiple
    /// runs of the same circuit are to be done, call `reset()` between
    /// executions.
    pub fn execute(&mut self)
    {
        for op in self.ops.iter()
        {
            match *op
            {
                CircuitOp::Gate(ref gate, ref bits) => {
                    self.q_state.apply_gate(&**gate, bits.as_slice());
                },
                CircuitOp::Measure(qbit, cbit)          => {
                    let msr = self.q_state.measure(qbit);
                    self.c_state.row_mut(cbit).assign(&msr);
                }
            }
        }
    }

    /// Create a histogram of measurements.
    ///
    /// Create a histogram of the measured classical bits. The `n` bits in the
    /// classical register are collected in a single `u64` integer value. The
    /// most significant bit `n-1` in the histogram key corresponds to the first
    /// bit in the classical register, and the least significant bit in the key
    /// to the last bit in the register. This function of course only works
    /// when there are at most 64 bits in the register. If there are more, use
    /// `histogram_string()`.
    pub fn histogram(&self) -> ::std::collections::HashMap<u64, usize>
    {
        let mut res = ::std::collections::HashMap::new();
        for col in self.c_state.gencolumns()
        {
            let key = col.iter().fold(0, |k, &b| (k << 1) | b as u64);
            let count = res.entry(key).or_insert(0);
            *count += 1;
        }
        res
    }

    /// Create a histogram of measurements.
    ///
    /// Create a histogram of the measured classical bits. The `n` bits in the
    /// classical register are collected in a single `usize` integer value,
    /// which is used as an index in a vector. The vector is of length
    /// `2`<sub>`n`</sub>, so use this function only for reasonably small
    /// numbers of `n`. For sparse collections, using `histogram()` or
    /// `histogram_string` may be better.
    pub fn histogram_vec(&self) -> Vec<usize>
    {
        let mut res = vec![0; self.c_state.cols()];
        for col in self.c_state.gencolumns()
        {
            let key = col.iter().fold(0, |k, &b| (k << 1) | b as usize);
            res[key] += 1;
        }
        res
    }

    /// Create a histogram of measurements.
    ///
    /// Create a histogram of the measured classical bits. The `n` bits in the
    /// classical register are collected in a string key, with the first character
    /// in the key corresponding to the first bit in the classical register.
    pub fn histogram_string(&self) -> ::std::collections::HashMap<String, usize>
    {
        let mut res = ::std::collections::HashMap::new();
        for col in self.c_state.gencolumns()
        {
            let key = col.iter().map(|&b| ::std::char::from_digit(b as u32, 10).unwrap()).collect();
            let count = res.entry(key).or_insert(0);
            *count += 1;
        }
        res
    }
}

#[cfg(test)]
mod tests
{
    use circuit::Circuit;
    use gates;

    #[test]
    fn test_execute()
    {
        let mut circuit = Circuit::new(2, 2, 5);
        circuit.add_gate(gates::X::new(), &[0]);
        circuit.add_gate(gates::X::new(), &[1]);
        circuit.add_gate(gates::CX::new(), &[0, 1]);
        circuit.add_measurement(0, 0);
        circuit.add_measurement(1, 1);
        circuit.execute();
        assert_eq!(circuit.cstate(), &array![[1, 1, 1, 1, 1], [0, 0, 0, 0, 0]]);
    }

    #[test]
    fn test_histogram()
    {
        let nr_shots = 4096;
        // chance of individual count being less than min_count is less than 10^-5
        // (assuming normal distribution)
        let min_count = 906;

        let mut circuit = Circuit::new(2, 2, nr_shots);
        circuit.add_gate(gates::H::new(), &[0]);
        circuit.add_gate(gates::H::new(), &[1]);
        circuit.add_measurement(0, 0);
        circuit.add_measurement(1, 1);
        circuit.execute();

        let hist = circuit.histogram();
        // With this many shots, we expect all keys to be present
        let mut keys: Vec<&String> = hist.keys().collect();
        keys.sort();
        assert_eq!(keys, vec!["00", "01", "10", "11"]);

        assert_eq!(hist.values().sum::<usize>(), nr_shots);
        assert!(*hist.values().min().unwrap() >= min_count);
    }
}