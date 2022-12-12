## Introductio n
ThisREADME explains how the virtual memory is planned at the moment after the
kernel loaded its paging-hierarchie.

## Plan
- Kernel memory:
    - First: HHDM start = 0x0 physically
    - Everything kernel related *has* to be somewhere above HHDM
    - Each physical addr which contains a page frame is equals to
      HHDM + <physical offset starting from 0x0>

      Example:
        Assuming the following physical memory chunks:
            - Chunk 1: 0x100 - 0x150
            - Chunk 2: 0x175 - 0x228
        
        In order to access those in VirtAddr-Space, you'd need to access:
            - Chunk 1: HDDM + 0x100 - HDDM + 0x150
            - Chunk 2: HDDM + 0x175 - HDDM + 0x228

Problem:
    Sei A der erste memory chunk und sei die Anzahl an page-frames 
    (von der größe her) mehr als ein benötigter memory chunk.
    Wie kann ich das nun machen, sodass der page frame allocator denkt, dass er
    einfach seinen stack und array hintereinander hat?

    => Mappe die ersten frames auf HHDM... aber WIE?
    Antwort:
        Wir verwenden den PhysMemMapper, der nur *temporär* verwendet wird.
        Wir verwenden seine Abstraktion, um alles vor der ganzen paging
        geschichte zu erstellen.

        Sobald paging ready ist, soll dieser *nicht mehr* verwendet werden,
        weil wir dann ja unseren page-frame-allocator haben.
