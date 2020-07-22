// This will contain logic for reading in input
use crate::errors::DeviceNotFoundError;
use crate::frequency_modulation::NaiveFrequencyModulation;
use crate::traits::BirdReceiver;
use cpal::traits::{DeviceTrait, HostTrait};
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct BirdListener {}

impl BirdListener {
    pub fn default() -> Self {
        Self {}
    }

    fn setup_receiver(&self) -> Result<(), Box<dyn std::error::Error>> {
        let device = cpal::default_host()
            .default_input_device()
            .ok_or(DeviceNotFoundError)?;
        let config = device.default_input_config()?;

        todo!()
    }

    fn open_receive<T>(sender: Sender<T>) {
        todo!()
    }

    /* Potentially useful reference code
    pub fn create_handler_fn<K: Strategy, T: cpal::Sample>(
        mut strategy: K,
    ) -> impl FnMut(&[T], &cpal::InputCallbackInfo) {
        return move |data: &[T], _: &cpal::InputCallbackInfo| {
            let _result = strategy.decode_bits(data);
        };
    }


    pub fn recv<K: 'static + Strategy + Send>(&self, strategy: K) {
        let config: cpal::SupportedStreamConfig =
            self.device.default_input_config().unwrap().into();

        let err_fn = |err| println!("{}", err);
        let input_stream = match config.sample_format() {
            cpal::SampleFormat::F32 => self.device.build_input_stream(
                &config.config(),
                BirdIInput::create_handler_fn::<K, f32>(strategy),
                err_fn,
            ),
            cpal::SampleFormat::I16 => self.device.build_input_stream(
                &config.config(),
                BirdIInput::create_handler_fn::<K, i16>(strategy),
                err_fn,
            ),
            cpal::SampleFormat::U16 => self.device.build_input_stream(
                &config.config(),
                BirdIInput::create_handler_fn::<K, u16>(strategy),
                err_fn,
            ),
        }
        .unwrap();
        input_stream.play().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
    */
}
use std::thread;
impl BirdReceiver<NaiveFrequencyModulation> for BirdListener {
    type BitFormat = u8;
    fn start(
        &self,
        strategy: NaiveFrequencyModulation,
    ) -> Result<Receiver<Self::BitFormat>, Box<dyn std::error::Error>> {
        let (sender, receiver) = channel::<Self::BitFormat>();
        let values = self.setup_receiver();
        let handler = thread::spawn(move || {
            Self::open_receive(sender);
        });

        todo!()
    }

    fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}
