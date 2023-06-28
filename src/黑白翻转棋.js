var flipChess = function (chessboar) {
    let chessboardArr = chessboar.map((x) => x.split(''));
    let temp = chessboardArr.map((x) => new Array(x.length).fill(0));
    function down(i, j) {
      let dir = [
        [0, 1],
        [1, 0],
        [0, -1],
        [-1, 0],
        [1, 1],
        [1, -1],
        [-1, 1],
        [-1, -1],
      ];
  
      for (let d of dir) {
        let path = [];
        let next_i = i;
        let next_j = j;
        while (1) {
          next_i = next_i + d[0];
          next_j = next_j + d[1];
          // console.log(next_i, next_j);
          // if (i === 2 && j === 2) {
          //   console.log(1);
          // }
          if (
            next_i >= 0 &&
            next_i < chessboardArr.length &&
            next_j >= 0 &&
            next_j < chessboardArr[i].length
          ) {
            if (chessboardArr[next_i][next_j] == '.') {
              break;
            }
            if (chessboardArr[next_i][next_j] == 'O') {
              path.push([next_i, next_j]);
              continue;
            }
            if (chessboardArr[next_i][next_j] == 'X') {
              for (let p of path) {
                chessboardArr[p[0]][p[1]] = 'X';
              }
              for (let p of path) {
                temp[i][j] = temp[i][j] + down(p[0], p[1]);
              }
              temp[i][j] = temp[i][j] + path.length;
              break;
            } else {
              break;
            }
          } else {
            break;
          }
        }
      }
      return temp[i][j];
    }
    let ans = -Infinity;
    for (let i = 0; i < chessboardArr.length; i++) {
      for (let j = 0; j < chessboardArr[i].length; j++) {
        if (chessboardArr[i][j] == '.') {
          temp.forEach((x) => {
            for (let i = 0; i < x.length; i++) {
              x[i] = 0;
            }
          });
          chessboardArr = chessboar.map((x) => x.split(''));
          down(i, j);
          ans = Math.max(ans, temp[i][j]);
        }
      }
    }
    // console.log(chessboardArr, temp);
    return ans;
  };