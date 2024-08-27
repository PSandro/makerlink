# makerlink

MakerSpace Autonomy

---

### Introduction

MakerLink is a mix of ERP, CRM and member portal - basically the brain of a modern MakerSpace. It's purpose is to establish a *link* between your MakerSpace CREW, the members, the machines and other resources like rooms or the coffee machine.

### Motivation
In a MakerSpace, creativity should never be interrupted by cumbersome booking systems or slow access controls. MakerLink ensures smooth, quick access to resources, so members can focus on innovation without compromising security, safety or accountability.

### Features
- machine reservation
- invoicing
- access control
- machine training management

### System Overview
```
┌──────────────────────────────────────────────┐          
│┌────────────────┐      ┌────────────────────┐│          
││                │ API  │                    ││          
││   Morty        ┼──────►    Citadel         ││          
││                │      │                    ││          
││ (Web Frontend) │      │   (Backend)        ││ Public   
││                │      │                    ││          
│└────────────────┘      └───────┬▲───────────┘│          
│                                ││            │          
│                                ││            │          
└────────────────────────────────┼┼────────────┘          
                                 ││                       
┌────────────────────────────────┼┼────────────┐          
│                                ││            │          
│  ┌─────────┐             ┌─────▼└────────┐   │          
│  │IoT Relay┼────────────►│               │   │          
│  └─────────┘             │               │   │          
│                          │               │   │          
│  ┌──────────┐            │    Plumbus    │   │MakerSpace
│  │NFC Reader┼───────────►│   (Connector) │   │(on-prem) 
│  └──────────┘            │               │   │          
│  ┌──────┐                │               │   │          
│  │Screen├───────────────►└───────────────┘   │          
│  └──────┘                                    │          
│                                              │          
└──────────────────────────────────────────────┘          
```

### License
See [LICENSE](./LICENSE)
