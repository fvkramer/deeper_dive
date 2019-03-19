const handler = {
  get: (target, name) => (name in target ? target[name] : 42),
};

const p = new Proxy({}, handler);
p.a = 1;
console.log(p.a, p.b);
