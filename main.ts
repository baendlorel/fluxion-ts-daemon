/**
 * Sample main.ts file for testing fluxion-daemon
 * This file simulates a process that crashes occasionally
 */

console.log('🚀 Starting application...');
console.log('📝 This is a sample TypeScript application for testing fluxion-daemon');
console.log('🔄 The daemon will automatically restart this process if it crashes\n');

let counter = 0;
let crashInterval = 15; // Crash every 15 seconds

// Main application logic
setInterval(() => {
  counter++;

  const timestamp = new Date().toISOString();
  console.log(`[${timestamp}] Application running... Counter: ${counter}`);

  // Simulate a crash periodically to test daemon restart functionality
  if (counter % crashInterval === 0) {
    console.log('💥 Simulating a crash to test daemon auto-restart...');
    console.log('🔄 The daemon should restart this process automatically.\n');

    // Throw an unhandled error to crash the process
    throw new Error('Simulated crash for testing daemon restart');
  }

  // Simulate some work
  if (counter % 5 === 0) {
    console.log('⚙️  Performing some work...');
  }

}, 2000); // Run every 2 seconds

// Handle graceful shutdown
process.on('SIGTERM', () => {
  console.log('🛑 Received SIGTERM, shutting down gracefully...');
  process.exit(0);
});

process.on('SIGINT', () => {
  console.log('🛑 Received SIGINT, shutting down gracefully...');
  process.exit(0);
});

console.log('✅ Application initialized successfully');
console.log('⌨️  Press Ctrl+C to stop (this will trigger daemon restart)');
console.log('📊 Watch the daemon keep this process alive!\n');
