let env = {};

env.memory = new WebAssembly.Memory({ initial: 1 });

env.calloc = function (num, size) {
  const ptr = env.memory.buffer.byteLength;
  env.memory.grow(num * size);
  return ptr;
};

export const calloc = env.calloc;

export default env;
