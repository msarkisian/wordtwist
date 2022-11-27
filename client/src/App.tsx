function App() {
  const dummyGrid = [
    ['f', 'e', 'b', 'r', 'n'],
    ['n', 'o', 'e', 'b', 'h'],
    ['n', 'v', 'r', 'e', 'g'],
    ['t', 'e', 'm', 'c', 'b'],
    ['r', 'f', 'y', 'l', 'v'],
  ];

  return (
    <div
      id="gameGrid"
      style={{
        display: 'grid',
        gridTemplateColumns: '1fr '.repeat(dummyGrid.length),
      }}
    >
      {dummyGrid.map((row) => row.map((column) => <div>{column}</div>))}
    </div>
  );
}

export default App;
