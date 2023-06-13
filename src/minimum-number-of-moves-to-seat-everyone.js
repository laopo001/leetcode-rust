/**
 * @param {number[]} seats
 * @param {number[]} students
 * @return {number}
 */
//  88 ms, faster than 56.29% 
var minMovesToSeat = function (seats, students) {
    seats = seats.sort((a, b) => a - b);
    students = students.sort((a, b) => a - b);
    let res = 0;
    for (let i = 0; i < seats.length; i++) {
        res += Math.abs(seats[i] - students[i])
    }
    return res
};

console.log(
    minMovesToSeat([3, 1, 5],
        [2, 7, 4])
);