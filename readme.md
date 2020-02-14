# Keyboards in Concert


Olav has some electronic keyboards and would like to play a tune. Unfortunately all of Olav’s keyboards are broken so each of them can only play some of the notes. By switching which instrument he is using he will be able to play the whole tune, but moving keyboards around is annoying so he would like to minimize the amount of times he has to switch. Can you help Olav figure out the minimum number of keyboard switches needed to play the entire song?


## Input

The first line of input is two space separated integers; n (1 <= n <= 1 000) the number of instruments, and m (1 <= m <= 1 000), the number of notes in the tune. This is followed by n lines, each starting with an integer k<sub> i</sub> (1 <= k<sub> i</sub> <= 1 000), the number of notes playable by instrument i, followed by k<sub> i</sub> pairwise distinct integers _l_ <sub>1</sub>, _l_ <sub>2</sub>, ... , _l_<sub>k<sub> i</sub></sub>, the notes that instrument i can play (1 <= _l_ <sub> j</sub> <= 1 000). Finally, there is a line with m space-separated integers – the notes of the tune in order.

## Output

The minimum number of times Olav needs to switch the instrument he is using during the tune.

<table class="sample" summary="sample data">

<tbody>

<tr>

<th>Sample Input 1</th>

<th>Sample Output 1</th>

</tr>

<tr>

<td>

<pre>2 10
2 1 2
2 2 3
1 2 1 2 3 3 2 3 1 3
</pre>

</td>

<td>

<pre>3
</pre>

</td>

</tr>

</tbody>

</table>



