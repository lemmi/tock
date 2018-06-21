
use kernel::common::StaticRef;
use kernel::common::regs::{self, ReadOnly, ReadWrite, WriteOnly};
    /// Analog to Digital Converter
#[repr(C)]
struct SaadcRegisters {
/// Start the ADC and prepare the result buffer in RAM
tasks_start: WriteOnly<u32, TASKS_START::Register>,
/// Take one ADC sample, if scan is enabled all channels are sampled
tasks_sample: WriteOnly<u32, TASKS_SAMPLE::Register>,
/// Stop the ADC and terminate any on-going conversion
tasks_stop: WriteOnly<u32, TASKS_STOP::Register>,
/// Starts offset auto-calibration
tasks_calibrateoffset: WriteOnly<u32, TASKS_CALIBRATEOFFSET::Register>,
_reserved0: [u8; 240],
/// The ADC has started
events_started: ReadWrite<u32, EVENTS_STARTED::Register>,
/// The ADC has filled up the Result buffer
events_end: ReadWrite<u32, EVENTS_END::Register>,
/// A conversion task has been completed. Depending on the mode, multiple conversion
events_done: ReadWrite<u32, EVENTS_DONE::Register>,
/// A result is ready to get transferred to RAM
events_resultdone: ReadWrite<u32, EVENTS_RESULTDONE::Register>,
/// Calibration is complete
events_calibratedone: ReadWrite<u32, EVENTS_CALIBRATEDONE::Register>,
/// The ADC has stopped
events_stopped: ReadWrite<u32, EVENTS_STOPPED::Register>,
_reserved1: [u8; 488],
/// Enable or disable interrupt
inten: ReadWrite<u32, INTEN::Register>,
/// Enable interrupt
intenset: ReadWrite<u32, INTENSET::Register>,
/// Disable interrupt
intenclr: ReadWrite<u32, INTENCLR::Register>,
_reserved2: [u8; 244],
/// Status
status: ReadOnly<u32>,
_reserved3: [u8; 252],
/// Enable or disable ADC
enable: ReadWrite<u32>,
_reserved4: [u8; 236],
/// Resolution configuration
resolution: ReadWrite<u32>,
/// Oversampling configuration. OVERSAMPLE should not be combined with SCAN. The RES
oversample: ReadWrite<u32>,
/// Controls normal or continuous sample rate
samplerate: ReadWrite<u32, SAMPLERATE::Register>,
}
register_bitfields![u32,
INTEN [
    /// Enable or disable interrupt on EVENTS_STARTED event
    STARTED OFFSET(0) NUMBITS(1) [
        /// Disable
        Disable = 0,
        /// Enable
        Enable = 1
    ],
    /// Enable or disable interrupt on EVENTS_END event
    END OFFSET(1) NUMBITS(1) [
        /// Disable
        Disable = 0,
        /// Enable
        Enable = 1
    ],
    /// Enable or disable interrupt on EVENTS_DONE event
    DONE OFFSET(2) NUMBITS(1) [
        /// Disable
        Disable = 0,
        /// Enable
        Enable = 1
    ],
    /// Enable or disable interrupt on EVENTS_RESULTDONE event
    RESULTDONE OFFSET(3) NUMBITS(1) [
        /// Disable
        Disable = 0,
        /// Enable
        Enable = 1
    ],
    /// Enable or disable interrupt on EVENTS_CALIBRATEDONE event
    CALIBRATEDONE OFFSET(4) NUMBITS(1) [
        /// Disable
        Disable = 0,
        /// Enable
        Enable = 1
    ],
    /// Enable or disable interrupt on EVENTS_STOPPED event
    STOPPED OFFSET(5) NUMBITS(1) [
        /// Disable
        Disable = 0,
        /// Enable
        Enable = 1
    ],
    /// Enable or disable interrupt on EVENTS_CH[0].LIMITH event
    CH0LIMITH OFFSET(6) NUMBITS(1) [
        /// Disable
        Disable = 0,
        /// Enable
        Enable = 1
    ],
    /// Enable or disable interrupt on EVENTS_CH[0].LIMITL event
    CH0LIMITL OFFSET(7) NUMBITS(1) [
        /// Disable
        Disable = 0,
        /// Enable
        Enable = 1
    ],
    /// Enable or disable interrupt on EVENTS_CH[1].LIMITH event
    CH1LIMITH OFFSET(8) NUMBITS(1) [
        /// Disable
        Disable = 0,
        /// Enable
        Enable = 1
    ],
    /// Enable or disable interrupt on EVENTS_CH[1].LIMITL event
    CH1LIMITL OFFSET(9) NUMBITS(1) [
        /// Disable
        Disable = 0,
        /// Enable
        Enable = 1
    ],
    /// Enable or disable interrupt on EVENTS_CH[2].LIMITH event
    CH2LIMITH OFFSET(10) NUMBITS(1) [
        /// Disable
        Disable = 0,
        /// Enable
        Enable = 1
    ],
    /// Enable or disable interrupt on EVENTS_CH[2].LIMITL event
    CH2LIMITL OFFSET(11) NUMBITS(1) [
        /// Disable
        Disable = 0,
        /// Enable
        Enable = 1
    ],
    /// Enable or disable interrupt on EVENTS_CH[3].LIMITH event
    CH3LIMITH OFFSET(12) NUMBITS(1) [
        /// Disable
        Disable = 0,
        /// Enable
        Enable = 1
    ],
    /// Enable or disable interrupt on EVENTS_CH[3].LIMITL event
    CH3LIMITL OFFSET(13) NUMBITS(1) [
        /// Disable
        Disable = 0,
        /// Enable
        Enable = 1
    ],
    /// Enable or disable interrupt on EVENTS_CH[4].LIMITH event
    CH4LIMITH OFFSET(14) NUMBITS(1) [
        /// Disable
        Disable = 0,
        /// Enable
        Enable = 1
    ],
    /// Enable or disable interrupt on EVENTS_CH[4].LIMITL event
    CH4LIMITL OFFSET(15) NUMBITS(1) [
        /// Disable
        Disable = 0,
        /// Enable
        Enable = 1
    ],
    /// Enable or disable interrupt on EVENTS_CH[5].LIMITH event
    CH5LIMITH OFFSET(16) NUMBITS(1) [
        /// Disable
        Disable = 0,
        /// Enable
        Enable = 1
    ],
    /// Enable or disable interrupt on EVENTS_CH[5].LIMITL event
    CH5LIMITL OFFSET(17) NUMBITS(1) [
        /// Disable
        Disable = 0,
        /// Enable
        Enable = 1
    ],
    /// Enable or disable interrupt on EVENTS_CH[6].LIMITH event
    CH6LIMITH OFFSET(18) NUMBITS(1) [
        /// Disable
        Disable = 0,
        /// Enable
        Enable = 1
    ],
    /// Enable or disable interrupt on EVENTS_CH[6].LIMITL event
    CH6LIMITL OFFSET(19) NUMBITS(1) [
        /// Disable
        Disable = 0,
        /// Enable
        Enable = 1
    ],
    /// Enable or disable interrupt on EVENTS_CH[7].LIMITH event
    CH7LIMITH OFFSET(20) NUMBITS(1) [
        /// Disable
        Disable = 0,
        /// Enable
        Enable = 1
    ],
    /// Enable or disable interrupt on EVENTS_CH[7].LIMITL event
    CH7LIMITL OFFSET(21) NUMBITS(1) [
        /// Disable
        Disable = 0,
        /// Enable
        Enable = 1
    ]
],
INTENSET [
    /// Write '1' to Enable interrupt on EVENTS_STARTED event
    STARTED OFFSET(0) NUMBITS(1) [
        /// Read: Disabled
        ReadDisabled = 0,
        /// Read: Enabled
        ReadEnabled = 1,
        /// Enable
        Enable = 1
    ],
    /// Write '1' to Enable interrupt on EVENTS_END event
    END OFFSET(1) NUMBITS(1) [
        /// Read: Disabled
        ReadDisabled = 0,
        /// Read: Enabled
        ReadEnabled = 1,
        /// Enable
        Enable = 1
    ],
    /// Write '1' to Enable interrupt on EVENTS_DONE event
    DONE OFFSET(2) NUMBITS(1) [
        /// Read: Disabled
        ReadDisabled = 0,
        /// Read: Enabled
        ReadEnabled = 1,
        /// Enable
        Enable = 1
    ],
    /// Write '1' to Enable interrupt on EVENTS_RESULTDONE event
    RESULTDONE OFFSET(3) NUMBITS(1) [
        /// Read: Disabled
        ReadDisabled = 0,
        /// Read: Enabled
        ReadEnabled = 1,
        /// Enable
        Enable = 1
    ],
    /// Write '1' to Enable interrupt on EVENTS_CALIBRATEDONE event
    CALIBRATEDONE OFFSET(4) NUMBITS(1) [
        /// Read: Disabled
        ReadDisabled = 0,
        /// Read: Enabled
        ReadEnabled = 1,
        /// Enable
        Enable = 1
    ],
    /// Write '1' to Enable interrupt on EVENTS_STOPPED event
    STOPPED OFFSET(5) NUMBITS(1) [
        /// Read: Disabled
        ReadDisabled = 0,
        /// Read: Enabled
        ReadEnabled = 1,
        /// Enable
        Enable = 1
    ],
    /// Write '1' to Enable interrupt on EVENTS_CH[0].LIMITH event
    CH0LIMITH OFFSET(6) NUMBITS(1) [
        /// Read: Disabled
        ReadDisabled = 0,
        /// Read: Enabled
        ReadEnabled = 1,
        /// Enable
        Enable = 1
    ],
    /// Write '1' to Enable interrupt on EVENTS_CH[0].LIMITL event
    CH0LIMITL OFFSET(7) NUMBITS(1) [
        /// Read: Disabled
        ReadDisabled = 0,
        /// Read: Enabled
        ReadEnabled = 1,
        /// Enable
        Enable = 1
    ],
    /// Write '1' to Enable interrupt on EVENTS_CH[1].LIMITH event
    CH1LIMITH OFFSET(8) NUMBITS(1) [
        /// Read: Disabled
        ReadDisabled = 0,
        /// Read: Enabled
        ReadEnabled = 1,
        /// Enable
        Enable = 1
    ],
    /// Write '1' to Enable interrupt on EVENTS_CH[1].LIMITL event
    CH1LIMITL OFFSET(9) NUMBITS(1) [
        /// Read: Disabled
        ReadDisabled = 0,
        /// Read: Enabled
        ReadEnabled = 1,
        /// Enable
        Enable = 1
    ],
    /// Write '1' to Enable interrupt on EVENTS_CH[2].LIMITH event
    CH2LIMITH OFFSET(10) NUMBITS(1) [
        /// Read: Disabled
        ReadDisabled = 0,
        /// Read: Enabled
        ReadEnabled = 1,
        /// Enable
        Enable = 1
    ],
    /// Write '1' to Enable interrupt on EVENTS_CH[2].LIMITL event
    CH2LIMITL OFFSET(11) NUMBITS(1) [
        /// Read: Disabled
        ReadDisabled = 0,
        /// Read: Enabled
        ReadEnabled = 1,
        /// Enable
        Enable = 1
    ],
    /// Write '1' to Enable interrupt on EVENTS_CH[3].LIMITH event
    CH3LIMITH OFFSET(12) NUMBITS(1) [
        /// Read: Disabled
        ReadDisabled = 0,
        /// Read: Enabled
        ReadEnabled = 1,
        /// Enable
        Enable = 1
    ],
    /// Write '1' to Enable interrupt on EVENTS_CH[3].LIMITL event
    CH3LIMITL OFFSET(13) NUMBITS(1) [
        /// Read: Disabled
        ReadDisabled = 0,
        /// Read: Enabled
        ReadEnabled = 1,
        /// Enable
        Enable = 1
    ],
    /// Write '1' to Enable interrupt on EVENTS_CH[4].LIMITH event
    CH4LIMITH OFFSET(14) NUMBITS(1) [
        /// Read: Disabled
        ReadDisabled = 0,
        /// Read: Enabled
        ReadEnabled = 1,
        /// Enable
        Enable = 1
    ],
    /// Write '1' to Enable interrupt on EVENTS_CH[4].LIMITL event
    CH4LIMITL OFFSET(15) NUMBITS(1) [
        /// Read: Disabled
        ReadDisabled = 0,
        /// Read: Enabled
        ReadEnabled = 1,
        /// Enable
        Enable = 1
    ],
    /// Write '1' to Enable interrupt on EVENTS_CH[5].LIMITH event
    CH5LIMITH OFFSET(16) NUMBITS(1) [
        /// Read: Disabled
        ReadDisabled = 0,
        /// Read: Enabled
        ReadEnabled = 1,
        /// Enable
        Enable = 1
    ],
    /// Write '1' to Enable interrupt on EVENTS_CH[5].LIMITL event
    CH5LIMITL OFFSET(17) NUMBITS(1) [
        /// Read: Disabled
        ReadDisabled = 0,
        /// Read: Enabled
        ReadEnabled = 1,
        /// Enable
        Enable = 1
    ],
    /// Write '1' to Enable interrupt on EVENTS_CH[6].LIMITH event
    CH6LIMITH OFFSET(18) NUMBITS(1) [
        /// Read: Disabled
        ReadDisabled = 0,
        /// Read: Enabled
        ReadEnabled = 1,
        /// Enable
        Enable = 1
    ],
    /// Write '1' to Enable interrupt on EVENTS_CH[6].LIMITL event
    CH6LIMITL OFFSET(19) NUMBITS(1) [
        /// Read: Disabled
        ReadDisabled = 0,
        /// Read: Enabled
        ReadEnabled = 1,
        /// Enable
        Enable = 1
    ],
    /// Write '1' to Enable interrupt on EVENTS_CH[7].LIMITH event
    CH7LIMITH OFFSET(20) NUMBITS(1) [
        /// Read: Disabled
        ReadDisabled = 0,
        /// Read: Enabled
        ReadEnabled = 1,
        /// Enable
        Enable = 1
    ],
    /// Write '1' to Enable interrupt on EVENTS_CH[7].LIMITL event
    CH7LIMITL OFFSET(21) NUMBITS(1) [
        /// Read: Disabled
        ReadDisabled = 0,
        /// Read: Enabled
        ReadEnabled = 1,
        /// Enable
        Enable = 1
    ]
],
INTENCLR [
    /// Write '1' to Clear interrupt on EVENTS_STARTED event
    STARTED OFFSET(0) NUMBITS(1) [
        /// Read: Disabled
        ReadDisabled = 0,
        /// Read: Enabled
        ReadEnabled = 1,
        /// Disable
        Disable = 1
    ],
    /// Write '1' to Clear interrupt on EVENTS_END event
    END OFFSET(1) NUMBITS(1) [
        /// Read: Disabled
        ReadDisabled = 0,
        /// Read: Enabled
        ReadEnabled = 1,
        /// Disable
        Disable = 1
    ],
    /// Write '1' to Clear interrupt on EVENTS_DONE event
    DONE OFFSET(2) NUMBITS(1) [
        /// Read: Disabled
        ReadDisabled = 0,
        /// Read: Enabled
        ReadEnabled = 1,
        /// Disable
        Disable = 1
    ],
    /// Write '1' to Clear interrupt on EVENTS_RESULTDONE event
    RESULTDONE OFFSET(3) NUMBITS(1) [
        /// Read: Disabled
        ReadDisabled = 0,
        /// Read: Enabled
        ReadEnabled = 1,
        /// Disable
        Disable = 1
    ],
    /// Write '1' to Clear interrupt on EVENTS_CALIBRATEDONE event
    CALIBRATEDONE OFFSET(4) NUMBITS(1) [
        /// Read: Disabled
        ReadDisabled = 0,
        /// Read: Enabled
        ReadEnabled = 1,
        /// Disable
        Disable = 1
    ],
    /// Write '1' to Clear interrupt on EVENTS_STOPPED event
    STOPPED OFFSET(5) NUMBITS(1) [
        /// Read: Disabled
        ReadDisabled = 0,
        /// Read: Enabled
        ReadEnabled = 1,
        /// Disable
        Disable = 1
    ],
    /// Write '1' to Clear interrupt on EVENTS_CH[0].LIMITH event
    CH0LIMITH OFFSET(6) NUMBITS(1) [
        /// Read: Disabled
        ReadDisabled = 0,
        /// Read: Enabled
        ReadEnabled = 1,
        /// Disable
        Disable = 1
    ],
    /// Write '1' to Clear interrupt on EVENTS_CH[0].LIMITL event
    CH0LIMITL OFFSET(7) NUMBITS(1) [
        /// Read: Disabled
        ReadDisabled = 0,
        /// Read: Enabled
        ReadEnabled = 1,
        /// Disable
        Disable = 1
    ],
    /// Write '1' to Clear interrupt on EVENTS_CH[1].LIMITH event
    CH1LIMITH OFFSET(8) NUMBITS(1) [
        /// Read: Disabled
        ReadDisabled = 0,
        /// Read: Enabled
        ReadEnabled = 1,
        /// Disable
        Disable = 1
    ],
    /// Write '1' to Clear interrupt on EVENTS_CH[1].LIMITL event
    CH1LIMITL OFFSET(9) NUMBITS(1) [
        /// Read: Disabled
        ReadDisabled = 0,
        /// Read: Enabled
        ReadEnabled = 1,
        /// Disable
        Disable = 1
    ],
    /// Write '1' to Clear interrupt on EVENTS_CH[2].LIMITH event
    CH2LIMITH OFFSET(10) NUMBITS(1) [
        /// Read: Disabled
        ReadDisabled = 0,
        /// Read: Enabled
        ReadEnabled = 1,
        /// Disable
        Disable = 1
    ],
    /// Write '1' to Clear interrupt on EVENTS_CH[2].LIMITL event
    CH2LIMITL OFFSET(11) NUMBITS(1) [
        /// Read: Disabled
        ReadDisabled = 0,
        /// Read: Enabled
        ReadEnabled = 1,
        /// Disable
        Disable = 1
    ],
    /// Write '1' to Clear interrupt on EVENTS_CH[3].LIMITH event
    CH3LIMITH OFFSET(12) NUMBITS(1) [
        /// Read: Disabled
        ReadDisabled = 0,
        /// Read: Enabled
        ReadEnabled = 1,
        /// Disable
        Disable = 1
    ],
    /// Write '1' to Clear interrupt on EVENTS_CH[3].LIMITL event
    CH3LIMITL OFFSET(13) NUMBITS(1) [
        /// Read: Disabled
        ReadDisabled = 0,
        /// Read: Enabled
        ReadEnabled = 1,
        /// Disable
        Disable = 1
    ],
    /// Write '1' to Clear interrupt on EVENTS_CH[4].LIMITH event
    CH4LIMITH OFFSET(14) NUMBITS(1) [
        /// Read: Disabled
        ReadDisabled = 0,
        /// Read: Enabled
        ReadEnabled = 1,
        /// Disable
        Disable = 1
    ],
    /// Write '1' to Clear interrupt on EVENTS_CH[4].LIMITL event
    CH4LIMITL OFFSET(15) NUMBITS(1) [
        /// Read: Disabled
        ReadDisabled = 0,
        /// Read: Enabled
        ReadEnabled = 1,
        /// Disable
        Disable = 1
    ],
    /// Write '1' to Clear interrupt on EVENTS_CH[5].LIMITH event
    CH5LIMITH OFFSET(16) NUMBITS(1) [
        /// Read: Disabled
        ReadDisabled = 0,
        /// Read: Enabled
        ReadEnabled = 1,
        /// Disable
        Disable = 1
    ],
    /// Write '1' to Clear interrupt on EVENTS_CH[5].LIMITL event
    CH5LIMITL OFFSET(17) NUMBITS(1) [
        /// Read: Disabled
        ReadDisabled = 0,
        /// Read: Enabled
        ReadEnabled = 1,
        /// Disable
        Disable = 1
    ],
    /// Write '1' to Clear interrupt on EVENTS_CH[6].LIMITH event
    CH6LIMITH OFFSET(18) NUMBITS(1) [
        /// Read: Disabled
        ReadDisabled = 0,
        /// Read: Enabled
        ReadEnabled = 1,
        /// Disable
        Disable = 1
    ],
    /// Write '1' to Clear interrupt on EVENTS_CH[6].LIMITL event
    CH6LIMITL OFFSET(19) NUMBITS(1) [
        /// Read: Disabled
        ReadDisabled = 0,
        /// Read: Enabled
        ReadEnabled = 1,
        /// Disable
        Disable = 1
    ],
    /// Write '1' to Clear interrupt on EVENTS_CH[7].LIMITH event
    CH7LIMITH OFFSET(20) NUMBITS(1) [
        /// Read: Disabled
        ReadDisabled = 0,
        /// Read: Enabled
        ReadEnabled = 1,
        /// Disable
        Disable = 1
    ],
    /// Write '1' to Clear interrupt on EVENTS_CH[7].LIMITL event
    CH7LIMITL OFFSET(21) NUMBITS(1) [
        /// Read: Disabled
        ReadDisabled = 0,
        /// Read: Enabled
        ReadEnabled = 1,
        /// Disable
        Disable = 1
    ]
],
SAMPLERATE [
    /// Capture and compare value. Sample rate is 16 MHz/CC
    CC OFFSET(0) NUMBITS(11) [],
    /// Select mode for sample rate control
    MODE OFFSET(12) NUMBITS(1) [
        /// Rate is controlled from SAMPLE task
        RateIsControlledFromSAMPLETask = 0,
        /// Rate is controlled from local timer (use CC to control the rate)
        RateIsControlledFromLocalTimerUseCCToControlTheRate = 1
    ]
]
];
const SAADC_BASE: StaticRef<SaadcRegisters> =
    unsafe { StaticRef::new(0x40007000 as *const SaadcRegisters) };
