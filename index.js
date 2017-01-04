
const suite = new require('benchmark').Suite({name: "FFI Test"});

const simple = require('./simple.js');
const complex = require('./complex.js');

suite
.add("Simple with JSON", simple)
.add("Complex Types", complex)
.on('cycle', function(event) {
  console.log(String(event.target));
})
.on('complete', function() {
  console.log('Fastest is ' + this.filter('fastest').map('name'));
})

.run()