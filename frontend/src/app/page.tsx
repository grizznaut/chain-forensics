import Search from '@/app/ui/search';

export default function Home() { 
  return (
    <main className="flex flex-col items-center justify-between p-24">
      <div className="w-full">
        <div className="flex w-full items-center justify-between">
          <h1 className="text-2xl">Address Search</h1>
        </div>
        <div className="mt-4 flex items-center justify-between gap-2 md:mt-8">
          <Search placeholder="Search for Address..." />
        </div>
        {/*  <Suspense key={query + currentPage} fallback={<InvoicesTableSkeleton />}>
          <Table query={query} currentPage={currentPage} />
        </Suspense> */}
        <div className="mt-5 flex w-full justify-center">
          {/* <Pagination totalPages={totalPages} /> */}
        </div>
      </div>
    </main>
  );
}
