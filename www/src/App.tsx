import React from 'react';
import Editor from 'react-simple-code-editor';
import { highlight, languages } from 'prismjs/components/prism-core';
import 'prismjs/components/prism-nasm';
import 'prismjs/themes/prism.css';
import init, {
  compile,
  init_vm,
  read_reg,
  get_cycle,
  execute,
  single_step,
} from '../wasm/pkg/wasm';

function App() {
  const registers = [
    'IP',
    'RF',
    'R0',
    'R1',
    'R2',
    'R3',
    'R4',
    'R5',
    'R6',
    'R7',
    'R8',
    'R9',
    'R10',
    'R11',
    'R12',
    'R13',
    'R14',
    'R15',
  ];
  const specialRegs = registers.slice(0, 2);
  const gpRegs = registers.slice(2, 18);

  const [code, setCode] = React.useState(`; Fibonacci(n)
; input = n = r1
; output = r0
    mov r1, 48

    mov     r5, 1
    cmp     r1, r5
    jle     base_case     ; if (n <= 1) return n

    xor     r2, r2        ; a = 0
    mov     r4, 1         ; b = 1
    mov     r3, 2         ; i = 2

loop_start:
    cmp     r3, r1
    jg      done         ; if (i > n) break

    add     r2, r4       ; temp = a + b
    mov     r5, r4       ; r5 = old_b
    mov     r4, r2       ; b = temp
    mov     r2, r5       ; a = old_b

    inc     r3           ; i++
    jmp     loop_start

done:
    mov     r0, r4       ; return b
    exit

base_case:
    mov     r0, r1       ; return n (r0 = n)
    exit
`);
  const [bytecode, setBytecode] = React.useState(new Uint8Array());
  const [cycle, setCycle] = React.useState<bigint>(0n);
  const [regState, setRegState] = React.useState<Array<bigint>>(
    new Array(registers.length).fill(0)
  );
  const [exitReached, setExitReached] = React.useState(false);

  const formatHex = (num: bigint) => {
    return num.toString(16).toUpperCase().padStart(16, '0');
  };
  const formatBinary = (arr: Uint8Array): string => {
    return Array.from(arr)
      .map((num) => num.toString(16).padStart(2, '0').toUpperCase())
      .join(' ')
      .replace(/00/g, '00 ');
  };

  const updateCycle = () => {
    setCycle(get_cycle());
  };
  const updateRegs = () => {
    registers.forEach((_, index) => {
      setRegState((prevState) => {
        const newState = [...prevState];
        newState[index] = read_reg(index);
        return newState;
      });
    });
  };
  const onClickCompile = () => {
    try {
      const bc = compile(code);
      setBytecode(bc);
      init_vm(bc);
      setExitReached(false);
      updateRegs();
      updateCycle();
    } catch (e) {
      alert(`Unable to compile: ${e}`);
    }
  };
  const onClickRun = () => {
    if (!bytecode.length) {
      alert('Bytecode is zero. You may not compiled it yet.');
    } else {
      try {
        execute();
      } catch (e) {
        alert(e);
      }
      updateRegs();
      updateCycle();
    }
  };
  const onClickSingleStep = () => {
    if (!bytecode.length) {
      alert('Bytecode is zero. You may not compiled it yet.');
    } else {
      try {
        single_step();
      } catch (e) {
        if (e === 'Exit') {
          setExitReached(true);
        } else {
          alert(e);
        }
      }
      updateRegs();
      updateCycle();
    }
  };

  React.useEffect(() => {
    init();
  }, []);

  return (
    <>
      <div className="flex justify-center items-center min-h-[100vh] min-w-[100vw]">
        <div className="flex flex-row flex-wrap p-4">
          <div className="flex flex-col p-4">
            <div className="flex flex-col gap-2 max-w-[400px]">
              <div className="flex flex-row gap-1">
                <button
                  onClick={onClickCompile}
                  type="button"
                  className="disabled:cursor-not-allowed select-none cursor-pointer text-bold w-fit focus:outline-none text-white bg-green-700 hover:bg-green-800 focus:ring-2 focus:ring-green-500 font-medium rounded-sm text-sm px-2.5 py-1.5 me-2 mb-2 dark:bg-green-600 dark:hover:bg-green-700 dark:focus:ring-green-800">
                  Compile
                </button>
                <button
                  disabled={exitReached}
                  onClick={onClickRun}
                  type="button"
                  className="disabled:cursor-not-allowed select-none cursor-pointer text-bold w-fit focus:outline-none text-white bg-green-700 hover:bg-green-800 focus:ring-2 focus:ring-green-500 font-medium rounded-sm text-sm px-2.5 py-1.5 me-2 mb-2 dark:bg-green-600 dark:hover:bg-green-700 dark:focus:ring-green-800">
                  Run
                </button>
                <button
                  disabled={exitReached}
                  onClick={onClickSingleStep}
                  type="button"
                  className="disabled:cursor-not-allowed select-none cursor-pointer text-bold w-fit focus:outline-none text-white bg-gray-700 hover:bg-gray-800 focus:ring-2 focus:ring-gray-500 font-medium rounded-sm text-sm px-2.5 py-1.5 me-2 mb-2 dark:bg-green-600 dark:hover:bg-green-700 dark:focus:ring-green-800">
                  Single Step
                </button>
              </div>

              <div className="no-scrollbar max-h-[600px] overflow-scroll">
                <Editor
                  value={code}
                  onValueChange={(code) => setCode(code)}
                  highlight={(code) => highlight(code, languages.nasm, 'nasm')}
                  padding={10}
                  className="min-w-[300px] bg-gray-400/10 font-[JetBrains_Mono] rounded-sm text-sm"
                />
              </div>
            </div>
          </div>

          <div className="flex flex-col p-4">
            <div className="flex flex-col gap-2">
              <div className="py-[4px] px-[0px] flex flex-row gap-x-2 font-[JetBrains_Mono] tracking-wider text-lg">
                <p className="bg-gray-400/10 text-center min-w-[70px]">CYCLE</p>
                <div>{cycle.toString()}</div>
              </div>

              <div className="flex flex-col">
                {specialRegs.map((reg, index) => (
                  <div
                    key={index}
                    className="py-[4px] px-[0px] flex flex-row gap-x-2 font-[JetBrains_Mono] tracking-wider text-lg">
                    <p className="bg-gray-400/10 text-center min-w-[70px]">
                      {reg}
                    </p>
                    <div>{formatHex(regState[registers.indexOf(reg)])}</div>
                  </div>
                ))}
              </div>

              <div className="flex flex-row flex-wrap gap-2">
                <div className="flex flex-col">
                  {gpRegs.slice(0, 8).map((reg, index) => (
                    <div
                      key={index}
                      className="py-[4px] px-[0px] flex flex-row gap-x-2 font-[JetBrains_Mono] tracking-wider text-lg">
                      <p className="bg-gray-400/10 text-center min-w-[70px]">
                        {reg}
                      </p>
                      <div>{formatHex(regState[registers.indexOf(reg)])}</div>
                    </div>
                  ))}
                </div>
                <div className="flex flex-col">
                  {gpRegs.slice(8, 16).map((reg, index) => (
                    <div
                      key={index}
                      className="py-[4px] px-[0px] flex flex-row gap-x-2 font-[JetBrains_Mono] tracking-wider text-lg">
                      <p className="bg-gray-400/10 text-center min-w-[70px]">
                        {reg}
                      </p>
                      <div>{formatHex(regState[registers.indexOf(reg)])}</div>
                    </div>
                  ))}
                </div>
              </div>

              <div className="flex flex-col gap-1">
                {bytecode.length > 0 && (
                  <>
                    <span>{`Bytecode (${bytecode.length} bytes):`}</span>
                    <span className="font-[JetBrains_Mono] max-w-[400px]">
                      {formatBinary(bytecode)}
                    </span>
                  </>
                )}
              </div>
            </div>
          </div>
        </div>
      </div>
    </>
  );
}

export default App;
