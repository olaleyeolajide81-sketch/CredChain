import { createSlice, createAsyncThunk, PayloadAction } from '@reduxjs/toolkit';

interface CreditProfile {
  user_address: string;
  credit_score: number;
  reputation_score: number;
  transaction_count: number;
  repayment_history: boolean[];
  last_updated: number;
  verification_status: boolean;
}

interface CreditState {
  profile: CreditProfile | null;
  isLoading: boolean;
  error: string | null;
  scoreFactors: Array<{ name: string; value: number }>;
}

const initialState: CreditState = {
  profile: null,
  isLoading: false,
  error: null,
  scoreFactors: [],
};

// Async thunks
export const fetchCreditProfile = createAsyncThunk(
  'credit/fetchProfile',
  async (userAddress: string) => {
    // API call to fetch credit profile
    const response = await fetch(`/api/credit/profile/${userAddress}`);
    if (!response.ok) {
      throw new Error('Failed to fetch credit profile');
    }
    return response.json();
  }
);

export const createCreditProfile = createAsyncThunk(
  'credit/createProfile',
  async (userAddress: string) => {
    // API call to create credit profile
    const response = await fetch('/api/credit/profile', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ user_address: userAddress }),
    });
    if (!response.ok) {
      throw new Error('Failed to create credit profile');
    }
    return response.json();
  }
);

export const fetchScoreFactors = createAsyncThunk(
  'credit/fetchScoreFactors',
  async (userAddress: string) => {
    const response = await fetch(`/api/credit/factors/${userAddress}`);
    if (!response.ok) {
      throw new Error('Failed to fetch score factors');
    }
    return response.json();
  }
);

const creditSlice = createSlice({
  name: 'credit',
  initialState,
  reducers: {
    clearError: (state: CreditState) => {
      state.error = null;
    },
    updateProfile: (state: CreditState, action: PayloadAction<Partial<CreditProfile>>) => {
      if (state.profile) {
        state.profile = { ...state.profile, ...action.payload };
      }
    },
  },
  extraReducers: (builder: any) => {
    builder
      .addCase(fetchCreditProfile.pending, (state: CreditState) => {
        state.isLoading = true;
        state.error = null;
      })
      .addCase(fetchCreditProfile.fulfilled, (state: CreditState, action: any) => {
        state.isLoading = false;
        state.profile = action.payload;
      })
      .addCase(fetchCreditProfile.rejected, (state: CreditState, action: any) => {
        state.isLoading = false;
        state.error = action.error.message || 'Failed to fetch credit profile';
      })
      .addCase(createCreditProfile.fulfilled, (state: CreditState, action: any) => {
        state.profile = action.payload;
      })
      .addCase(fetchScoreFactors.fulfilled, (state: CreditState, action: any) => {
        state.scoreFactors = action.payload;
      });
  },
});

export const { clearError, updateProfile } = creditSlice.actions;
export default creditSlice.reducer;
