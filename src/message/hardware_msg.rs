use driver_rust::elevio::poll::CallButton;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct
ConvertedCallButton
{
    pub floor: u8,
    pub call: u8,
}

#[derive(Serialize, Deserialize)]
pub enum
HardwareData
{
    DataCallButton      {call_button_data: ConvertedCallButton},
    DataFloorSensor     {floor: u8},
    DataStopButton      {status: bool},
    DataObstruction     {status: bool},
    SetMotorDirection   {dir: u8},
    SetCallButtonLight  {floor: u8, call: u8, status: bool},
    SetDoorLight        {status: bool},
    SetStopLight        {status: bool},
    SetFloorIndicator   {floor: u8},
    PASS
}

impl 
ConvertedCallButton 
{
    pub fn
    from_call_button(call_button: CallButton) -> Self
    {
        Self{
            floor: call_button.floor,
            call: call_button.call,
        }
    }
}
