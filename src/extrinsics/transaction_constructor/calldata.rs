pub struct CallData(Vec<u8>);

impl CallData {
    pub fn get(&self) -> &Vec<u8> {
        self.0.as_ref()
    }

    pub fn to_vec(self) -> Vec<u8> {
        self.0
    }
}

impl From<Vec<u8>> for CallData {
    fn from(val: Vec<u8>) -> Self {
        CallData(val)
    }
}

impl From<CallData> for Vec<u8> {
    fn from(val: CallData) -> Self {
        val.0
    }
}
