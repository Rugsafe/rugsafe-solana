// init
const camera = new THREE.PerspectiveCamera(70, window.innerWidth / window.innerHeight, 0.01, 10);
camera.position.z = 3;

const scene = new THREE.Scene();

// lighting
const light = new THREE.DirectionalLight(0xffffff, 1);
light.position.set(1, 1, 1).normalize();
scene.add(light);

// load fbx model
const loader = new THREE.FBXLoader();
// loader.load('https://threejs.org/examples/models/fbx/Samba%20Dancing.fbx', function (object) {
loader.load('http://localhost:4000/src/1.fbx', function (object) {
  // scale the model if necessary
  object.scale.set(0.01, 0.01, 0.01); // Adjust the scale based on your model's size

  scene.add(object);
}, undefined, function (error) {
  console.error('An error happened while loading the FBX model:', error);
});

// renderer
const renderer = new THREE.WebGLRenderer({ antialias: true });
renderer.setSize(window.innerWidth, window.innerHeight);
document.body.appendChild(renderer.domElement);

// animation loop
function animation(time) {
  // Optional: rotate the scene or specific objects
  scene.rotation.y += 0.01;

  renderer.render(scene, camera);
}

renderer.setAnimationLoop(animation);
