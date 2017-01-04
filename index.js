
const Benchmark = new require('benchmark');
Benchmark.options.minSamples = 100;
Benchmark.options.maxTime = 10;

const suite = Benchmark.Suite({name: "FFI Test"});

const simple = require('./simple.js');
const complex = require('./complex.js');

suite
.add("Simple with JSON", simple)
.add("Complex Types", complex)
.on('cycle', function(event) {
  console.log(String(event.target));
})
.on('complete', function() {
  console.log("---------------------------------");
  console.log('Fastest is ' + this.filter('fastest').map('name'));
})

.run({'async': true})