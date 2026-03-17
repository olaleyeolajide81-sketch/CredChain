import { createSlice, createAsyncThunk, PayloadAction } from '@reduxjs/toolkit';

interface LoanRecord {
  borrower: string;
  lender: string;
  amount: number;
  interest_rate: number;
  term_days: number;
  start_date: number;
  repayment_status: boolean;
  collateral?: string;
}

interface LoansState {
  userLoans: LoanRecord[];
  availableLoans: LoanRecord[];
  isLoading: boolean;
  error: string | null;
}

const initialState: LoansState = {
  userLoans: [],
  availableLoans: [],
  isLoading: false,
  error: null,
};

// Async thunks
export const fetchUserLoans = createAsyncThunk(
  'loans/fetchUserLoans',
  async (userAddress: string) => {
    const response = await fetch(`/api/loans/user/${userAddress}`);
    if (!response.ok) {
      throw new Error('Failed to fetch user loans');
    }
    return response.json();
  }
);

export const createLoan = createAsyncThunk(
  'loans/createLoan',
  async (loanData: {
    borrower: string;
    lender: string;
    amount: number;
    interest_rate: number;
    term_days: number;
    collateral?: string;
  }) => {
    const response = await fetch('/api/loans', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(loanData),
    });
    if (!response.ok) {
      throw new Error('Failed to create loan');
    }
    return response.json();
  }
);

export const repayLoan = createAsyncThunk(
  'loans/repayLoan',
  async (loanId: string) => {
    const response = await fetch(`/api/loans/${loanId}/repay`, {
      method: 'POST',
    });
    if (!response.ok) {
      throw new Error('Failed to repay loan');
    }
    return { loanId };
  }
);

const loansSlice = createSlice({
  name: 'loans',
  initialState,
  reducers: {
    clearError: (state: LoansState) => {
      state.error = null;
    },
    updateLoanStatus: (state: LoansState, action: PayloadAction<{ loanId: string; status: boolean }>) => {
      const loan = state.userLoans.find(l => l.borrower === action.payload.loanId);
      if (loan) {
        loan.repayment_status = action.payload.status;
      }
    },
  },
  extraReducers: (builder: any) => {
    builder
      .addCase(fetchUserLoans.pending, (state: LoansState) => {
        state.isLoading = true;
        state.error = null;
      })
      .addCase(fetchUserLoans.fulfilled, (state: LoansState, action: any) => {
        state.isLoading = false;
        state.userLoans = action.payload;
      })
      .addCase(fetchUserLoans.rejected, (state: LoansState, action: any) => {
        state.isLoading = false;
        state.error = action.error.message || 'Failed to fetch loans';
      })
      .addCase(createLoan.fulfilled, (state: LoansState, action: any) => {
        state.userLoans.push(action.payload);
      })
      .addCase(repayLoan.fulfilled, (state: LoansState, action: any) => {
        const loan = state.userLoans.find(l => l.borrower === action.payload.loanId);
        if (loan) {
          loan.repayment_status = true;
        }
      });
  },
});

export const { clearError, updateLoanStatus } = loansSlice.actions;
export default loansSlice.reducer;
