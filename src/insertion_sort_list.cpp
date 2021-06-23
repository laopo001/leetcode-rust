/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */

struct ListNode
{
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};
//  56 ms, faster than 19.60% 
#include <climits>
class Solution
{
public:
    ListNode *insertionSortList(ListNode *head)
    {
        ListNode *res = new ListNode(INT_MIN);
        ListNode *res_c = res;
        ListNode *head_c = head;
        while (head_c != nullptr)
        {
            res_c = res;
            while (res_c != nullptr)
            {
                if ((res_c->val <= head_c->val && res_c->next == nullptr) || (res_c->val <= head_c->val && res_c->next != nullptr && res_c->next->val > head_c->val))
                {
                    auto next = res_c->next;
                    res_c->next = new ListNode(head_c->val);
                    res_c->next->next = next;
                    break;
                }
                res_c = res_c->next;
            }
            head_c = head_c->next;
        }
        return res->next;
    }
};