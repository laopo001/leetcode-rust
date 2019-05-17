let longestConsecutive = (nums) => {
    let map = {};
    for (let n of nums) {
        map[n] = 1;
    }
    let res = 0;
    // tslint:disable-next-line:forin
    for (let k in map) {
        let i = parseInt(k, 10);
        if (map[i] === 0) {
            continue;
        }
        while (map[i + 1] != null) {
            // console.log(map[i + 1])
            if (map[i + 1] === 1) {
                map[k] += 1;
                map[i + 1] = 0;
                i++;
            } else {
                map[k] += map[i + 1];
                map[i + 1] = 0;
                break;
            }
        }
        res = Math.max(map[k], res);
    }
    return res;
};

