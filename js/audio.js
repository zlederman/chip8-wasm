const play = (frequency = 300, duration = 1e3) => {
    const context = new AudioContext();
    const gainNode = context.createGain();
    const oscillator = context.createOscillator();
    oscillator.frequency.value = frequency;
    oscillator.connect(gainNode);
    gainNode.connect(context.destination);
    oscillator.start(0);
    setTimeout(() => oscillator.stop(), duration);
  };
  