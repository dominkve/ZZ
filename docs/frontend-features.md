# Frontend Features for Zaoζάω早

## 1. Core User Flows

### **1.1 Code Submission**
- 3 modes:
    - Playground:
        - User names the files as they wish.
        - User can choose compiler and version.
        - User can add custom flags to execution command.
        - Possibly access to a shell
        - User can upload own files
        - Resource limits are non-configurable for now
        - Network acess is prohibited for all for now
    - Run:
        - Runs with a few predefined test cases or user defined test cases
        - Users can write a small amount of custom test cases
        - Problem should either have predetermined all test cases, or only core ones and the rest is randomized, randomized and user ones would have their outputs determined by running a known working efficient solution
        - Returns results, comparing actual and expected output
    - Submit:
        - Runs against all test cases.
        - Returns first failed test case with detailed logs.
        - Returns runtime and memory usage as overall for now.
        - Stores results.

### **1.2 View Submission History**
- Users can see past submissions.
- Each submission shows:
  - Execution status (passed, failed, error).
  - Runtime and memory usage.
  - Output and errors.
  - Test case results.

### **1.3 Problem Browsing & Selection**
- Users can browse available problems.
- Problems are categorized by:
  - Difficulty (Easy, Medium, Hard)
  - Tags (e.g., "Dynamic Programming", "Sorting")
  - Language support
- Users can select a problem to view its details.

### **1.4 Test Case Management**
- Users can create custom test cases for problems.
- Inputs and expected outputs are specified manually.
- Admins can create and manage official test cases.

### **1.5 Account & Authentication (Future Enhancement)**
- Users log in with credentials (or OAuth if needed later).
- Track progress across problems.
- Save preferred language settings.

---

## 2. API Endpoints Used

### **2.1 Submission API** (`POST /playground`)
- Request:
  ```json
  {
    "files": [
        {
            "filename": "main.cpp",
            "compiler": "g++ 10.2.1",
            "compiler_flags": None,
        }
    ]
  }
  ```
- Response:
  ```json
  {
    "status": "success",
    "output": "Passed all.",
    "execution_time": 1.23,
    "memory_usage": 12
  }
  ```

### **2.1 Submission API** (`POST /run`)
- Request:
  ```json
  {
    "code": "print(42)",
    "language": "python",
    "problem": "Binary Search",
    "test_cases": {}
  }
  ```
- Response:
  ```json
  {
    "status": "success",
    "output": {
        "error": None,
        "test_cases": [
            {            
            "id": 1,
            "status": "fail",
            "output": 5,
            "expected_output": 10
            },
            {            
            "id": 2,
            "status": "sucess",
            "output": 6,
            "expected_output": 6
            }
        ]
    },
  }
  ```

### **2.1 Submission API** (`POST /submit`)
- Request:
  ```json
  {
    "code": "print(42)",
    "language": "python",
    "problem": "Binary Search"
  }
  ```
- Response:
  ```json
  {
    "status": "success",
    "output": "Passed all.",
    "execution_time": 1.23,
    "memory_usage": 12
  }
  ```

### **2.2 History API** (`GET /history?user_id=123`)
- Response:
  ```json
  {
    "submissions": [
      {
        "id": 1,
        "status": "passed",
        "output": "42"
      }
    ]
  }
  ```

### **2.3 Problem API** (`GET /problems`)
- Response:
  ```json
  {
    "problems": [
      {
        "id": 10,
        "title": "Factorial Calculator",
        "difficulty": "Easy"
      }
    ]
  }
  ```

### **2.4 Test Case API** (`GET /testcases?problem_id=10`)
- Response:
  ```json
  {
    "test_cases": [
      {
        "input": "5",
        "expected_output": "120"
      }
    ]
  }
  ```

---

## 3. Frontend Data Storage Strategy

### **3.1 Local Storage (Temporary Data)**
- Store last-selected language preference.
- Cache unsaved code (to prevent loss on refresh).
- Store UI preferences (dark mode, font size, etc.).

### **3.2 State Management (Session Data)**
- Track ongoing submission status (pending, running, completed).
- Store current problem details and test cases.

---

## 4. Future Enhancements (Not a Priority Now)
✅ **Live collaboration** – Multiple users edit the same code in real-time.
✅ **Real-time execution updates** – WebSockets for instant feedback.
✅ **Problem recommendations** – Suggests next problems based on history.

---

## 5. Development Notes
- **Framework**: Next.js (React-based)
- **Styling**: TailwindCSS
- **Editor**: Likely Monaco Editor (VS Code-like experience)
- **API Calls**: Fetch or Axios (simple for now, can replace later if needed)

### **To-Do Before Implementation**
- [ ] Finalize API specifications.
- [ ] Set up basic UI layout.
- [ ] Implement submission and result fetching.
- [ ] Implement history and problem listing.

---

**Notes:** This document is subject to change as the project evolves. Features should be implemented in the simplest way first, with optimizations coming later.