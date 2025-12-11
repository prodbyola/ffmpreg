io → container → codec → transform → codec → container → io

Todo

- Minimal WAV pipeline

  > End-to-end pipeline, audible audio.
  - [ ] Create project and basic folders (core, containers/wav, codecs/pcm, cli)
  - [ ] Implement Packet, Frame, Timebase
  - [ ] Read WAV and produce Packets (containers/wav/read.rs)
  - [ ] Write Packets back (containers/wav/write.rs)
  - [ ] PCM passthrough codec (decode → encode)
  - [ ] Connect pipeline: read → decode → encode → write
  - [ ] Minimal CLI: ffmpreg -i input.wav -o output.wav
  - [ ] Test with a simple WAV file

- Frame inspection / Media info

  > Show internal frame info, minimal ffprobe alternative.
  - [ ] Add CLI option --show
  - [ ] Iterate over Packets → Frames
  - [ ] Display pts, sample count, channels, sample rate
  - [ ] Test output with example WAV

- Basic transform

  > Apply simple operation on frames (e.g., gain)
  - [ ] Create transforms/gain.rs
  - [ ] Implement trait Transform<T>
  - [ ] Integrate pipeline: read → decode → transform → encode → write
  - [ ] CLI: ffmpreg -filter gain=2.0
  - [ ] Test amplified audio

- Multi-file / batch

  > Process multiple files using the same pipeline
  - [ ] CLI accepts multiple files or wildcard (folder/\*.wav)
  - [ ] Iterate files → pipeline
  - [ ] Create separate output for each file
  - [ ] Test with 2-3 WAV files

- More containers

  > Add raw video support (Y4M)
  - [ ] Create containers/y4m/read.rs and write.rs
  - [ ] Parse Y4M header (width, height, framerate, colorspace)
  - [ ] Produce Packets/Frames
  - [ ] Minimal pipeline: decode → encode → write
  - [ ] CLI: ffmpreg -i input.y4m -o output.y4m
  - [ ] Test with a Y4M file

- More codecs

  > ADPCM, multi-channel PCM
  - [ ] Add ADPCM codec
  - [ ] Support multi-channel PCM
  - [ ] Pipeline: decode → transform → encode → write
  - [ ] Roundtrip tests for each codec

- Chained filters
  > Apply multiple transforms in sequence
  - [ ] CLI: ffmpreg-filter gain=2.0 -filter normalize
  - [ ] Create transforms/normalize.rs
  - [ ] Pipeline applies filters in sequence
  - [ ] Test audio with two chained filters
