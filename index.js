
const Benchmark = new require('benchmark');
Benchmark.options.minSamples = 100;
Benchmark.options.maxTime = 10;

const suite = Benchmark.Suite({name: "FFI Test"});

const simple = require('./simple.js');
const complex = require('./complex.js');

const appInfo = {
	id: 'net.maidsafe.test.sample.benchmark',
	name: 'benchmark-test',
	vendor: 'maidsafe'
};
const containers = {
	'_public': ['Read'],
	'_pictures': ['Read', 'Insert'],
	// '_app/net.maidsafe.test.benchmark-app': ['ManagePermissions', 'Read', 'Update']
};

// simple(appInfo, containers);
// complex(appInfo, containers);


suite
.add("Simple with JSON", function() {
	simple(appInfo, containers);
})
.add("Complex Types", function() {
	complex(appInfo, containers);
})
.on('cycle', function(event) {
  console.log(String(event.target));
})
.on('complete', function() {
  console.log("---------------------------------");
  console.log('Fastest is ' + this.filter('fastest').map('name'));
})

.run({'async': true})