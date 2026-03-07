use driver_rust::elevio::poll::CallButton;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum
HardwareData
{
    //DataCallButton      {call_button_data: CallButton},
    DataFloorSensor     {floor: u8},
    DataStopButton      {status: bool},
    DataObstruction     {status: bool},
    SetMotorDirection   {dir: u8},
    SetCallButtonLight  {floor: u8, call: u8, status: bool},
    SetDoorLight        {status: bool},
    SetStopLight        {status: bool},
    SetFloorIndicator   {floor: u8},
}
