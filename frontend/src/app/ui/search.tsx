'use client';
import { useState } from 'react';
 
export default function Search({ placeholder }: { placeholder: string }) {
  const [query, setQuery] = useState("");

  function handleSearch(term: string) {
    // fetch address stats
    fetch(`api/address/${term}`)
      .then(res => res.json())
    .then(
      (result) => {
          console.log(result);
      },
    )
    // fetch address txs
    fetch(`api/address/${term}/txs`)
    .then(res => res.json())
    .then(
      (result) => {
          console.log(result);
      },
    )
  }
 
  return (
    <div className="relative flex flex-1 flex-shrink-0">
      <label htmlFor="search" className="sr-only">
        Search
      </label>
      <div className="flex w-full gap-4 items-center justify-between">
        <input
          className="block w-full rounded-md border border-gray-200 py-[9px] pl-10 text-sm outline-2 text-gray-900 placeholder:text-gray-500"
          placeholder={placeholder}
          onChange={(e) => {setQuery(e.target.value)}}
        />
        <button className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
            onClick={(e) => {
            handleSearch(query);
          }}
        >
          Submit
        </button>
      </div>
    </div>
  );
}
