# Token Vesting Program

This is a decentralized token vesting application built on the Solana blockchain using the Anchor framework. It allows companies to create vesting accounts for their employees, manage token distribution, and allow employees to claim their tokens based on a vesting schedule.

## Features

1. **Create Vesting Account**: Allows the company to create a vesting account where tokens will be distributed to employees over time. The company also creates a treasury account to store the tokens.
   
2. **Create Employee Account**: Allows the employer to add an employee to the vesting program, setting their individual vesting schedule (start time, end time, cliff time, and total tokens).

3. **Claim Tokens**: Allows the employee to claim vested tokens, depending on the vesting schedule. The employee can only claim tokens once the cliff time has passed, and they can claim tokens proportionally based on the vesting schedule.

## Instructions

### 1. **Create Vesting Account**

This instruction is used to create a new vesting account for a company, where tokens will be distributed to employees.

- **Arguments**:
  - `company_name`: The name of the company (used as a seed for the vesting account).
  
- **Accounts involved**:
  - `signer`: The user creating the vesting account (the company's representative).
  - `vesting_account`: The account holding the vesting information.
  - `mint`: The SPL token mint associated with the tokens being vested.
  - `treasury_token_account`: The treasury account that holds the tokens to be distributed.
  - `system_program`: The system program (for initialization).
  - `token_program`: The token program used for token management.

---

### 2. **Create Employee Account**

This instruction is used by the employer to create a vesting account for an employee. The employee will have their own vesting schedule and will receive a certain amount of tokens over time.

- **Arguments**:
  - `start_time`: The vesting start time.
  - `end_time`: The vesting end time.
  - `total_amount`: The total amount of tokens assigned to the employee.
  - `cliff_time`: The time after which the employee can start claiming tokens.

- **Accounts involved**:
  - `owner`: The employer or creator of the employee's vesting account.
  - `beneficiary`: The employee.
  - `vesting_account`: The company's vesting account (associated with the employee).
  - `employee_account`: The employee’s account where their token distribution schedule is stored.
  - `system_program`: The system program (for initialization).

---

### 3. **Claim Tokens**

This instruction is used by an employee to claim tokens from their vesting account. The employee can only claim tokens after the cliff time has passed, and the amount they can claim is proportional to the vesting schedule.

- **Arguments**:
  - `_company_name`: The company name (for seed purposes).

- **Accounts involved**:
  - `beneficiary`: The employee claiming the tokens.
  - `employee_account`: The employee's account containing their vesting schedule and withdrawal history.
  - `vesting_account`: The company’s vesting account containing the employee's token distribution.
  - `mint`: The SPL token mint associated with the tokens being claimed.
  - `treasury_token_account`: The treasury account that holds the tokens to be distributed to employees.
  - `employee_token_account`: The employee's associated token account where the tokens will be sent.
  - `system_program`: The system program (for initialization).
  - `token_program`: The token program used for token management.
  - `associated_token_program`: The associated token program used for creating token accounts.

---

## Errors

The application defines several error codes to ensure correct behavior:

- `ClaimNotAvailableYet`: The employee cannot claim tokens yet (they are still within the cliff period).
- `InvalidVestingPeriod`: The vesting period is invalid (total vesting time cannot be zero).
- `CalculationOverflow`: There was an error in calculating the vested amount.
- `NothingToClaim`: The employee has no tokens available to claim at the moment.

---

## Data Structures

### **Vesting Account**
This account holds the information about the company's vesting program.

- `owner`: The company owner who has control over the vesting program.
- `mint`: The SPL token mint associated with the tokens.
- `treasury_token_account`: The token account where the tokens are held.
- `company_name`: The name of the company.
- `treasury_bump`: The bump seed used for the treasury account.
- `bump`: The bump seed used for the vesting account.

### **Employee Account**
This account holds the information about an employee's vesting schedule.

- `beneficiary`: The employee who will receive the tokens.
- `start_time`: The start time of the vesting.
- `end_time`: The end time of the vesting.
- `cliff_time`: The time after which the employee can start claiming tokens.
- `vesting_account`: The associated company’s vesting account.
- `total_amount`: The total amount of tokens assigned to the employee.
- `total_withdrawn`: The total amount of tokens the employee has already claimed.
- `bump`: The bump seed used for the employee account.

---

## Usage

1. **Create a vesting account**: A company creates a vesting account by calling the `create_vesting_account` function.
2. **Create an employee account**: The company then creates an employee account for each employee, specifying their vesting schedule.
3. **Claim tokens**: Employees can claim their tokens once the cliff time has passed, and the system calculates how much they can claim based on their vesting schedule.

---

## License

This project is licensed under the MIT License.

---
