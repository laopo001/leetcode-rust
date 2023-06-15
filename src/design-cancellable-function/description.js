var cancellable = function (generator) {
  let cancel;
  let generatorValue;
  let step = 0;
  let promise = new Promise((resolve, reject) => {
    cancel = () => {
      try {
        run(generator.throw('Cancelled'));
      } catch {
        reject('Cancelled');
      }
    };
    function run(result) {
      try {
        if (result.done) {
          resolve(result.value);
        } else {
          if (result.value instanceof Promise) {
            result.value
              .then((res) => {
                generatorValue = res;
                step++;
                run(generator.next(res));
              })
              .catch((err) => {
                try {
                  run(generator.throw(err));
                } catch {
                  reject(err);
                }
              });
          } else {
            generatorValue = result.value;
            step++;
            run(generator.next());
          }
        }
      } catch (err) {
        reject(err);
      }
    }
    run(generator.next());
  });
  return [cancel, promise];
};

/**
 * function* tasks() {
 *   const val = yield new Promise(resolve => resolve(2 + 2));
 *   yield new Promise(resolve => setTimeout(resolve, 100));
 *   return val + 1;
 * }
 * const [cancel, promise] = cancellable(tasks());
 * setTimeout(cancel, 50);
 * promise.catch(console.log); // logs "Cancelled" at t=50ms
 */