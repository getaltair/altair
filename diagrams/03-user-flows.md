# Altair User Flow Diagrams

## Quick Task Capture Flow (ADHD-Optimized)

```mermaid
flowchart TD
    START([User Has Idea]) --> TRIGGER{Where?}
    
    TRIGGER -->|Anywhere in App| HOTKEY[Press Ctrl+K]
    TRIGGER -->|On Task Page| QUICK[Click Quick Add]
    TRIGGER -->|Mobile| WIDGET[Tap Widget]
    
    HOTKEY --> MODAL[Quick Capture Modal]
    QUICK --> MODAL
    WIDGET --> MODAL
    
    MODAL --> TYPE[Type Task Title]
    TYPE --> ENTER{Press Enter?}
    
    ENTER -->|Yes| SAVE_MIN[Save with Defaults]
    ENTER -->|No| ADD_DETAILS{Add Details?}
    
    ADD_DETAILS -->|Yes| EXPAND[Expand Form]
    ADD_DETAILS -->|No| SAVE_MIN
    
    EXPAND --> OPTIONAL[Add Optional Info]
    OPTIONAL --> PROJECT[Select Project]
    OPTIONAL --> DUE[Set Due Date]
    OPTIONAL --> PRIORITY[Set Priority]
    OPTIONAL --> SAVE_FULL[Save Complete Task]
    
    SAVE_MIN --> LOCAL_DB[(Save to Local DB)]
    SAVE_FULL --> LOCAL_DB
    
    LOCAL_DB --> SHOW[Show in List Immediately]
    LOCAL_DB --> SYNC{Online?}
    
    SYNC -->|Yes| API_SAVE[POST to API]
    SYNC -->|No| QUEUE[Add to Sync Queue]
    
    API_SAVE --> SUCCESS{Success?}
    SUCCESS -->|Yes| UPDATE[Update Local with Server ID]
    SUCCESS -->|No| RETRY[Retry Queue]
    
    QUEUE --> WAIT[Wait for Connection]
    WAIT --> SYNC
    
    UPDATE --> DONE([Task Captured!])
    SHOW --> DONE
    
    style START fill:#60A5FA,stroke:#3B82F6,color:#fff
    style MODAL fill:#FB923C,stroke:#EA580C,color:#fff
    style SAVE_MIN fill:#14B8A6,stroke:#0D9488,color:#fff
    style DONE fill:#14B8A6,stroke:#0D9488,color:#fff
```

## AI Task Breakdown User Journey

```mermaid
flowchart TD
    START([User Creates Large Task]) --> DETECT{Task Complexity<br/>Detected?}
    
    DETECT -->|High| SUGGEST[Show AI Breakdown Button]
    DETECT -->|Low| SKIP[Skip Suggestion]
    
    SUGGEST --> USER_CLICK{User Clicks<br/>Breakdown?}
    
    USER_CLICK -->|Yes| LOADING[Show Loading State]
    USER_CLICK -->|No| MANUAL[User Can Add<br/>Subtasks Manually]
    
    LOADING --> AI_CALL[Call AI Service]
    AI_CALL --> LLM[Local LLM Processes]
    LLM --> PARSE[Parse Subtasks]
    PARSE --> PREVIEW[Show Preview Modal]
    
    PREVIEW --> REVIEW[User Reviews Suggestions]
    REVIEW --> DECIDE{Accept?}
    
    DECIDE -->|Accept All| CREATE_ALL[Create All Subtasks]
    DECIDE -->|Edit First| EDIT_MODAL[Show Edit Modal]
    DECIDE -->|Reject| MANUAL
    
    EDIT_MODAL --> MODIFY[User Modifies Subtasks]
    MODIFY --> CREATE_ALL
    
    CREATE_ALL --> SAVE_SUBS[Save to Database]
    SAVE_SUBS --> LINK[Link to Parent Task]
    LINK --> ESTIMATE[Calculate Total Estimate]
    ESTIMATE --> SHOW[Show in Task Tree]
    
    SHOW --> CELEBRATION[Show Success Animation]
    CELEBRATION --> DONE([Breakdown Complete!])
    
    style START fill:#60A5FA,stroke:#3B82F6,color:#fff
    style AI_CALL fill:#FB923C,stroke:#EA580C,color:#fff
    style PREVIEW fill:#60A5FA,stroke:#3B82F6,color:#fff
    style DONE fill:#14B8A6,stroke:#0D9488,color:#fff
```

## Focus Mode Session Flow

```mermaid
stateDiagram-v2
    [*] --> Idle
    
    Idle --> SelectTask: User Selects Task
    SelectTask --> ConfigureSession: Set Duration
    
    ConfigureSession --> FocusActive: Start Timer
    
    FocusActive --> Paused: User Pauses
    FocusActive --> Completed: Timer Ends
    FocusActive --> Interrupted: External Interruption
    
    Paused --> FocusActive: Resume
    Paused --> Cancelled: End Session
    
    Interrupted --> SaveContext: Auto-save State
    SaveContext --> ParkingLot: Show Parking Lot
    ParkingLot --> FocusActive: Return to Task
    ParkingLot --> Cancelled: Switch Tasks
    
    Completed --> BreakTime: Suggest Break
    BreakTime --> Idle: Break Complete
    
    Cancelled --> Idle
    
    note right of FocusActive
        - Hide distractions
        - Show timer
        - Track time
        - Gentle reminders
    end note
    
    note right of Interrupted
        - Save current work
        - Quick note capture
        - Easy return path
    end note
    
    note right of Completed
        - Celebrate completion
        - Log time
        - Update task
        - Suggest break
    end note
```

## Time Tracking User Flow

```mermaid
sequenceDiagram
    participant User
    participant UI
    participant Timer
    participant LocalDB
    participant API
    
    User->>UI: Click Start Timer on Task
    UI->>Timer: Initialize Timer
    Timer->>LocalDB: Create TimeEntry (start_time)
    UI->>User: Show Running Timer
    
    loop Every Second
        Timer->>UI: Update Display
    end
    
    Note over User,API: User Works on Task
    
    alt User Stops Manually
        User->>UI: Click Stop
        UI->>Timer: Stop Timer
    else Timer Auto-stops (Idle Detection)
        Timer->>Timer: Detect Idle
        Timer->>UI: Show "Still Working?" Modal
        UI->>User: Ask to Continue
        
        alt User Confirms
            User->>UI: Yes, Still Working
            UI->>Timer: Continue
        else User Idle/No Response
            Timer->>Timer: Auto-stop
        end
    end
    
    Timer->>Timer: Calculate Duration
    Timer->>LocalDB: Update TimeEntry (end_time, duration)
    Timer->>API: POST /time-entries
    API-->>Timer: Confirm
    
    Timer->>UI: Update Task Duration
    UI->>User: Show Completion Animation
    
    Note over User,API: Time Logged Successfully
```

## Project Dashboard Navigation

```mermaid
graph TD
    HOME[Home Dashboard] --> DECIDE{User Wants To...}
    
    DECIDE -->|View All Projects| PROJECTS[Projects List]
    DECIDE -->|Quick Capture| QUICK[Quick Add Modal]
    DECIDE -->|See Today's Tasks| TODAY[Today View]
    DECIDE -->|Check Time| TIME[Time Analytics]
    
    PROJECTS --> SELECT[Select Project]
    SELECT --> PROJ_VIEW[Project Detail View]
    
    PROJ_VIEW --> TABS{View Type}
    TABS -->|List| LIST[Task List]
    TABS -->|Board| KANBAN[Kanban Board]
    TABS -->|Calendar| CAL[Calendar View]
    TABS -->|Analytics| ANALYTICS[Progress Charts]
    
    LIST --> TASK_DETAIL[Click Task]
    KANBAN --> TASK_DETAIL
    CAL --> TASK_DETAIL
    
    TASK_DETAIL --> ACTIONS{Action}
    ACTIONS -->|Edit| EDIT[Edit Modal]
    ACTIONS -->|Start Timer| TIMER[Focus Session]
    ACTIONS -->|Add Note| NOTE[Quick Note]
    ACTIONS -->|Complete| COMPLETE[Mark Complete]
    
    COMPLETE --> ANIMATION[Celebration Animation]
    ANIMATION --> UPDATE[Update Views]
    UPDATE --> PROJ_VIEW
    
    style HOME fill:#60A5FA,stroke:#3B82F6,color:#fff
    style QUICK fill:#14B8A6,stroke:#0D9488,color:#fff
    style ANIMATION fill:#FB923C,stroke:#EA580C,color:#fff
```

## Mobile Quick Capture from Widget

```mermaid
flowchart LR
    LOCK[Lock Screen] --> WIDGET[Altair Widget]
    HOME[Home Screen] --> WIDGET
    
    WIDGET --> TAP[Tap Widget]
    TAP --> OPEN{App State?}
    
    OPEN -->|Closed| LAUNCH[Launch App]
    OPEN -->|Background| RESUME[Resume App]
    OPEN -->|Active| SHOW[Show App]
    
    LAUNCH --> CAPTURE[Quick Capture Screen]
    RESUME --> CAPTURE
    SHOW --> CAPTURE
    
    CAPTURE --> INPUT[Type/Voice Input]
    INPUT --> SAVE[Save Instantly]
    SAVE --> CONFIRM[Show Checkmark]
    CONFIRM --> CLOSE{Close?}
    
    CLOSE -->|Auto| MINIMIZE[Return to Widget]
    CLOSE -->|Manual| ADD_MORE[Add Another?]
    
    ADD_MORE -->|Yes| CAPTURE
    ADD_MORE -->|No| MINIMIZE
    
    MINIMIZE --> SUCCESS([Task Captured!])
    
    style WIDGET fill:#60A5FA,stroke:#3B82F6,color:#fff
    style CAPTURE fill:#14B8A6,stroke:#0D9488,color:#fff
    style SUCCESS fill:#14B8A6,stroke:#0D9488,color:#fff
```

## First-Time User Onboarding

```mermaid
flowchart TD
    START([New User Arrives]) --> LANDING[Landing Page]
    LANDING --> ACTION{Action}
    
    ACTION -->|Sign Up| REGISTER[Registration Form]
    ACTION -->|Learn More| FEATURES[Features Tour]
    
    REGISTER --> VERIFY[Email Verification]
    VERIFY --> LOGIN[First Login]
    
    LOGIN --> WELCOME[Welcome Screen]
    WELCOME --> TOUR{Take Tour?}
    
    TOUR -->|Yes| STEP1[Step 1: Quick Capture]
    TOUR -->|Skip| DASHBOARD
    
    STEP1 --> DEMO1[Interactive Demo]
    DEMO1 --> STEP2[Step 2: Task Organization]
    STEP2 --> DEMO2[Show Projects]
    DEMO2 --> STEP3[Step 3: Focus Mode]
    STEP3 --> DEMO3[Timer Demo]
    DEMO3 --> COMPLETE_TOUR[Tour Complete]
    
    COMPLETE_TOUR --> PROMPT[Create First Task?]
    PROMPT -->|Yes| FIRST_TASK[Quick Capture]
    PROMPT -->|Later| DASHBOARD
    
    FIRST_TASK --> SAVE_FIRST[Save Task]
    SAVE_FIRST --> CELEBRATE[🎉 Celebration]
    CELEBRATE --> DASHBOARD[Main Dashboard]
    
    DASHBOARD --> SIDEBAR[Show Sidebar]
    SIDEBAR --> HINTS[Contextual Hints]
    HINTS --> READY([Ready to Use!])
    
    style START fill:#60A5FA,stroke:#3B82F6,color:#fff
    style DEMO1 fill:#FB923C,stroke:#EA580C,color:#fff
    style CELEBRATE fill:#14B8A6,stroke:#0D9488,color:#fff
    style READY fill:#14B8A6,stroke:#0D9488,color:#fff
```

## Task Status Lifecycle

```mermaid
stateDiagram-v2
    [*] --> Backlog: Created
    
    Backlog --> Todo: Prioritized
    Backlog --> Archived: Not Needed
    
    Todo --> InProgress: Started
    Todo --> Blocked: Dependency
    Todo --> Backlog: Deprioritized
    
    InProgress --> Blocked: Hit Blocker
    InProgress --> Paused: Interrupted
    InProgress --> Review: Need Review
    InProgress --> Completed: Finished
    
    Blocked --> InProgress: Blocker Resolved
    Blocked --> Todo: Still Waiting
    
    Paused --> InProgress: Resumed
    Paused --> Todo: Context Lost
    
    Review --> InProgress: Changes Needed
    Review --> Completed: Approved
    
    Completed --> Archived: Old Task
    Completed --> Todo: Reopened
    
    Archived --> [*]
    
    note right of Backlog
        - No due date
        - Low priority
        - Someday/maybe
    end note
    
    note right of InProgress
        - Timer running
        - Focus mode
        - Time tracked
    end note
    
    note right of Completed
        - Time logged
        - Celebration shown
        - Stats updated
    end note
```

## Gamification Progress Flow

```mermaid
flowchart TD
    ACTION[User Completes Action] --> CHECK{Achievement<br/>Trigger?}
    
    CHECK -->|First Task| BADGE1[Award: First Steps]
    CHECK -->|7 Day Streak| BADGE2[Award: Week Warrior]
    CHECK -->|10 Hours Focus| BADGE3[Award: Deep Diver]
    CHECK -->|No Badge| STATS[Update Stats Only]
    
    BADGE1 --> MODAL[Show Badge Modal]
    BADGE2 --> MODAL
    BADGE3 --> MODAL
    
    MODAL --> ANIMATION[Play Animation]
    ANIMATION --> SAVE[Save to Profile]
    SAVE --> SHARE{Share?}
    
    SHARE -->|Yes| SOCIAL[Generate Share Image]
    SHARE -->|No| PROFILE
    
    SOCIAL --> PROFILE[Update Profile]
    STATS --> PROFILE
    
    PROFILE --> DASHBOARD[Return to Dashboard]
    DASHBOARD --> DONE([Dopamine Delivered!])
    
    style ACTION fill:#60A5FA,stroke:#3B82F6,color:#fff
    style MODAL fill:#FB923C,stroke:#EA580C,color:#fff
    style ANIMATION fill:#FB923C,stroke:#EA580C,color:#fff
    style DONE fill:#14B8A6,stroke:#0D9488,color:#fff
```

## Error Recovery Flow (ADHD-Friendly)

```mermaid
flowchart TD
    ERROR[Error Occurs] --> DETECT{Error Type}
    
    DETECT -->|Network| NET_ERR[Network Error]
    DETECT -->|Validation| VAL_ERR[Validation Error]
    DETECT -->|Server| SRV_ERR[Server Error]
    
    NET_ERR --> OFFLINE[Switch to Offline Mode]
    OFFLINE --> QUEUE[Queue Actions]
    QUEUE --> NOTIFY[Show Working Offline Badge]
    NOTIFY --> CONTINUE[User Continues Working]
    CONTINUE --> CHECK{Connection<br/>Restored?}
    CHECK -->|Yes| SYNC[Auto-sync Queue]
    CHECK -->|No| CONTINUE
    SYNC --> CLEAR[Clear Badge]
    CLEAR --> SUCCESS
    
    VAL_ERR --> HIGHLIGHT[Highlight Field]
    HIGHLIGHT --> EXPLAIN[Show Clear Error]
    EXPLAIN --> SUGGEST[Suggest Fix]
    SUGGEST --> RETRY[User Corrects]
    RETRY --> SUCCESS
    
    SRV_ERR --> LOG[Log Error]
    LOG --> FRIENDLY[Show Friendly Message]
    FRIENDLY --> OPTIONS{Severity}
    OPTIONS -->|Critical| CONTACT[Offer Support Contact]
    OPTIONS -->|Minor| RETRY_AUTO[Auto-retry]
    
    RETRY_AUTO --> RETRY_RESULT{Success?}
    RETRY_RESULT -->|Yes| SUCCESS
    RETRY_RESULT -->|No| CONTACT
    
    CONTACT --> REPORT[Easy Error Report]
    REPORT --> SUCCESS[User Can Continue]
    
    SUCCESS --> DONE([Recovered Gracefully])
    
    style ERROR fill:#EF4444,stroke:#DC2626,color:#fff
    style OFFLINE fill:#FB923C,stroke:#EA580C,color:#fff
    style SUCCESS fill:#14B8A6,stroke:#0D9488,color:#fff
    style DONE fill:#14B8A6,stroke:#0D9488,color:#fff
```

---

**Flow Design Principles:**

1. **Minimal Clicks** - Most actions in 1-2 steps
2. **Forgiving** - Easy undo, auto-save, recovery
3. **Clear Feedback** - Every action has visible response
4. **No Dead Ends** - Always a path forward
5. **Context Preservation** - Save state on interruption
6. **Visual Progress** - Show where user is in flow
7. **Escape Hatches** - Can always cancel/go back
8. **Smart Defaults** - Reduce decisions needed
