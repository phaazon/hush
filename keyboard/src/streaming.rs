use alto::{Buffer, Context, Mono, Source, SourceState, StreamingSource};
use hush::instrument::Instrument;
use hush::time::SampleTime;

const SAMPLE_FREQ: usize = 44100;
const SAMPLE_READAHEAD: usize = 44100; // one second ahead

// Handle audio streaming.
//
// An object of this type can be used to stream an instrument activity to the DSP device in full
// transparency.
pub struct Streamer {
  source: StreamingSource,
  buffers: Vec<Buffer>,
  processed_samples_nb: usize, // number of samples already processed
}

impl Streamer {
  pub fn new(al_ctx: &mut Context) -> Self {
    let processed_samples_nb = 0;
    let source = al_ctx.new_streaming_source().expect("OpenAL source");

    let buffers = (0..2).into_iter().map(|_| {
      al_ctx.new_buffer::<Mono<f32>, _>(&vec![0.; SAMPLE_FREQ], SAMPLE_FREQ as i32).unwrap()
    }).collect::<Vec<_>>();

    Self { source, buffers, processed_samples_nb }
  }

  /// Refresh the streaming process to check whether the DSP and/or streaming buffers should be
  /// updated.
  pub fn refresh<I>(&mut self, instrument: &mut I) where I: Instrument {
    // first thing first: we check the state of the DSP
    if self.source.state() == SourceState::Playing {
      if instrument.is_active() {
        // DSP playing and instrument is still active: we need to check whether some more data is
        // needed
        unimplemented!();
      } else {
        // DSP is playing but the instrument is not active anymore: release phase
        unimplemented!();
      }
    } else if instrument.is_active() {
      // the DSP is not playing but the instrument is active, so we need to queue at
      // least one buffer in
      self.queue_one_buffer(instrument); 
      self.source.play();
    } else {
      //unimplemented!(); // RESET
    }
  }

  /// Queue a buffer of data.
  fn queue_one_buffer<I>(&mut self, instrument: &mut I) where I: Instrument {
    // TODO: maybe add the possibility to add more buffer?
    let mut buffer = self.buffers.swap_remove(0);
    let start = self.processed_samples_nb;
    let end = start + SAMPLE_READAHEAD - 1;
    let samples = instrument.get_samples(SampleTime(start), SampleTime(end));

    // upload the samples to the DSP buffer
    buffer.set_data::<Mono<f32>, _>(samples, SAMPLE_FREQ as i32);

    // queue the buffer to the current DSP source
    self.source.queue_buffer(buffer);

    // update the number of samples already processed
    self.processed_samples_nb += SAMPLE_READAHEAD;
  }
}