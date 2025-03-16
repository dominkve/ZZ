<script lang="ts">
    import { onMount, onDestroy } from "svelte";

    let problemTable: HTMLElement;
    onMount(() => {
        problemTable = document.getElementById("problem-table") as HTMLElement;
    });
    onDestroy(() => {
        if (problemTable)
            problemTable.remove();
    });
</script>

<table id="problem-table" class="w-2xl text-left table-auto border-seperate border border-gray-400">
    <thead class="bg-gray-800 text-amber-50 text-xl">
        <tr>
          <th class="border border-gray-300 ..."></th>
          <th class="border border-gray-300 p-2 ...">Problem</th>
          <th class="border border-gray-300 p-2 text-center ...">Description</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td class="border border-gray-300 text-2xl p-2 ...">1.</td>
          <td class="border border-gray-300 text-xl p-2 ...">Sum</td>
          <td class="border border-gray-300 p-2 ...">Sum two numbers.</td>
        </tr>
        <tr>
          <td class="border border-gray-300 text-2xl p-2 ...">1.</td>
          <td class="border border-gray-300 text-xl p-2 ...">Sum</td>
          <td class="border border-gray-300 p-2 ...">Sum two numbers.</td>
        </tr>
        <tr>
          <td class="border border-gray-300 text-2xl p-2 ...">1.</td>
          <td class="border border-gray-300 text-xl p-2 ...">Sum</td>
          <td class="border border-gray-300 p-2 ...">Sum two numbers.</td>
        </tr>
        <tr>
          <td class="border border-gray-300 text-2xl p-2 ...">1.</td>
          <td class="border border-gray-300 text-xl p-2 ...">Sum</td>
          <td class="border border-gray-300 p-2 ...">Sum two numbers.</td>
        </tr>
        <tr>
          <td class="border border-gray-300 text-2xl p-2 ...">1.</td>
          <td class="border border-gray-300 text-xl p-2 ...">Sum</td>
          <td class="border border-gray-300 p-2 ...">Sum two numbers.</td>
        </tr>
        <tr>
          <td class="border border-gray-300 text-2xl p-2 ...">1.</td>
          <td class="border border-gray-300 text-xl p-2 ...">Sum</td>
          <td class="border border-gray-300 p-2 ...">Sum two numbers.</td>
        </tr>
        <tr>
          <td class="border border-gray-300 text-2xl p-2 ...">1.</td>
          <td class="border border-gray-300 text-xl p-2 ...">Sum</td>
          <td class="border border-gray-300 p-2 ...">Sum two numbers.</td>
        </tr>
        <tr>
          <td class="border border-gray-300 text-2xl p-2 ...">1.</td>
          <td class="border border-gray-300 text-xl p-2 ...">Sum</td>
          <td class="border border-gray-300 p-2 ...">Sum two numbers.</td>
        </tr>
        <tr>
          <td class="border border-gray-300 text-2xl p-2 ...">1.</td>
          <td class="border border-gray-300 text-xl p-2 ...">Sum</td>
          <td class="border border-gray-300 p-2 ...">Sum two numbers.</td>
        </tr>
        <tr>
          <td class="border border-gray-300 text-2xl p-2 ...">1.</td>
          <td class="border border-gray-300 text-xl p-2 ...">Sum</td>
          <td class="border border-gray-300 p-2 ...">Sum two numbers.</td>
        </tr>
        <tr>
          <td class="border border-gray-300 text-2xl p-2 ...">1.</td>
          <td class="border border-gray-300 text-xl p-2 ...">Sum</td>
          <td class="border border-gray-300 p-2 ...">Sum two numbers.</td>
        </tr>
        <tr>
          <td class="border border-gray-300 text-2xl p-2 ...">1.</td>
          <td class="border border-gray-300 text-xl p-2 ...">Sum</td>
          <td class="border border-gray-300 p-2 ...">Sum two numbers.</td>
        </tr>
        <tr>
          <td class="border border-gray-300 text-2xl p-2 ...">1.</td>
          <td class="border border-gray-300 text-xl p-2 ...">Sum</td>
          <td class="border border-gray-300 p-2 ...">Sum two numbers.</td>
        </tr>
      </tbody>
  </table>