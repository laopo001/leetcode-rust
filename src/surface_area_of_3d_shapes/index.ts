/**
 * File: c:\Users\35327\Githubs\leetcode-rust\src\surface_area_of_3d_shapes\index.ts
 * Project: c:\Users\35327\Githubs\leetcode-rust\src\surface_area_of_3d_shapes
 * Created Date: Sunday, April 21st 2019, 11:51:44 pm
 * @author: liaodh
 * @summary: short description for the file
 * -----
 * Last Modified: Monday, April 22nd 2019, 12:17:33 am
 * Modified By: liaodh
 * -----
 * Copyright (c) 2019 liaodh
 */

// fast than
var swapPairs = function (head) {
    let res = null;
    let last = null;
    while (head != null && head.next != null) {
        let a = head;
        let b = head.next;
        if (res == null) {
            res = b;
        }
        if (last != null) {
            last.next = b;
        }
        let c = head.next.next;
        b.next = a;
        a.next = c;
        head = c;
        last = a;

    }
    return res || head;
};